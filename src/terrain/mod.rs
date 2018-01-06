//! Module related to managing, drawing, and colliding with terrain.

mod mesh_gen;
mod voxel;
mod world_gen;

use std::collections::{HashMap, VecDeque};
use std::mem;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use luminance::framebuffer::Framebuffer;
use luminance::linear::M44;
use luminance::pipeline::{entry, pipeline, RenderState};
use luminance::texture::{Dim2, Flat};
use luminance::shader::program::{Program, ProgramError, Uniform, UniformBuilder,
                                 UniformInterface, UniformWarning};
use luminance_glfw::{Device, GLFWDevice};
use camera::Camera;
use maths::{Frustum, ToMatrix, Translation};
use model::Drawable;
use resources::Resources;
use shader;
use self::voxel::{Block, BlockList, Sector, SectorSpaceCoords};
use self::world_gen::WorldGen;

// Type of terrain position vertex attribute.
type Position = [f32; 3];

// Type of terrain texture coordinate attribute.
type UV = [f32; 2];

// Type of face attribute. Serves to replace the normal
// vector, since on a cube the normals always lie along
// an axis.
type FaceNum = u32;

// A terrain vertex.
type Vertex = (Position, UV, FaceNum);

/// The length of one side of a cubic sector.
pub const SECTOR_SIZE: usize = 32;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.75, 0.8, 1.0];
const COLLIDE_PADDING: f32 = 0.3;

/// Drawable manager for world terrain. Handles the rendering
/// of each sector.
pub struct Terrain<'a> {
    shader: Program<Vertex, (), Uniforms>,
    resources: &'a Resources,
    sectors: HashMap<(i32, i32, i32), Sector>,
    //needed: Arc<Mutex<HashMap<(i32, i32, i32), bool>>>,
    shared_info: SharedInfo,
    //nearby_rx: Receiver<Nearby>,
    //needed_tx: Sender<(i32, i32, i32)>,
    join_handle: Option<JoinHandle<()>>,
}

impl<'a> Terrain<'a> {
    /// Create a new `Terrain` using the shared `Resources`.
    /// # Panics
    /// This constructor panics if shaders fail to load.
    pub fn new(resources: &'a Resources) -> Terrain<'a> {
        let (shader, warnings) = Self::load_shaders().unwrap();
        for warn in &warnings {
            eprintln!("{:?}", warn);
        }
        
        //let shared_info = Arc::new(Mutex::new(Default::default()));
        
        let mut sectors = HashMap::with_capacity(5 * 5 * 5);
        //for dx in -2..3 {
        //    for dy in -2..3 {                
        //        for dz in -2..3 {
        //            let pos = (dx, dy, dz);
        //            
        //            sectors.insert(pos, Sector::new(resources, pos));
        //            
        //            //println!("pos: {:?}", pos);
        //        }
        //    }
        //}
        
        sectors.insert((0, 0, 0), Sector::new(resources, (0, 0, 0), BlockList::new([Block::Loam; SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE])));
        sectors.insert((1, 0, 0), Sector::new(resources, (1, 0, 0), BlockList::new([Block::Loam; SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE])));
        sectors.insert((0, 0, 1), Sector::new(resources, (0, 0, 1), BlockList::new([Block::Loam; SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE])));
        sectors.insert((1, 0, 1), Sector::new(resources, (1, 0, 1), BlockList::new([Block::Loam; SECTOR_SIZE * SECTOR_SIZE * SECTOR_SIZE])));
        
        //let (nearby_tx, nearby_rx) = mpsc::channel();
        //let (needed_tx, needed_rx) = mpsc::channel();
        //TerrainGenThread::new(shared_info.clone(), nearby_tx, needed_rx).spawn();
        
        Terrain {
            resources,
            sectors,
            //sectors: HashMap::new(),
            shader,
            //needed: HashMap::new(),
            shared_info: Arc::new(Mutex::new(Default::default())),
            //nearby_rx,
            //needed_tx,
            join_handle: None,
        }
    }
    
    /// Spawn the world generation thread.
    /// The terrain will immediately begin generating.
    pub fn spawn_generator(&mut self) {
        let shared_info = self.shared_info.clone();
        
        self.join_handle = Some(thread::spawn(move || {
            loop {
                let shared_info = shared_info.lock().unwrap();
                
                if shared_info.exiting {
                    return;
                }
                
                // ABSOLUTELY CRITICAL
                // to avoid deadlock
                mem::drop(shared_info);
                thread::sleep(Duration::from_millis(500));
                //
            }
        }));
    }
    
    // Stop generating the world, terminating and joining
    // the generation thread. This function is called by
    // `Terrain`'s `Drop` impl.
    // # Panics
    // Panics if the worldgen thread panicked.
    // # Blocking
    // This function blocks while joining and while waiting
    // to acquire a mutex.
    fn stop_generator(&mut self) {
        if let Some(handle) = self.join_handle.take() {
            println!("Stopping worldgen thread...");
            self.shared_info.lock().unwrap().exiting = true;
            println!("Aquired lock");
            
            handle.join().unwrap();
            println!("Joined worldgen thread");
        }
    }
    
    /// Perform a frame update.
    /// May block for some time until a mutex can be aquired.
    pub fn update(&mut self, camera: &Camera) {
        //self.shared_info.lock().unwrap().player_pos = translation.clone();
        
        let sector = sector_at(&camera.translation());
        self.sectors.retain(|&k, _| {
            let dx = k.0 as f32 - sector.0 as f32;
            let dy = k.1 as f32 - sector.1 as f32;
            let dz = k.2 as f32 - sector.2 as f32;
            
            let dist_sq = dx * dx + dy * dy + dz * dz;
            
            //println!("{}", dist_sq);
            
            dist_sq < 280.
        });
    }
    
    /// Adjust for collisions with the terrain.
    pub fn collide(&self, translation: &mut Translation) {
        {
            let back_t = Translation::new(translation.x, translation.y, translation.z.round() - 1.);
            let back = match self.get_visible_block(&back_t) {
                Some(b) => !b.is_air(),
                None => false,
            };
            
            let margin = back_t.z + 1. + COLLIDE_PADDING;
            if back && translation.z < margin {
                translation.z = margin;
            }
        }
        
        //
        
        {
            let front_t = Translation::new(translation.x, translation.y, translation.z.round() + 1.);
            let front = match self.get_visible_block(&front_t) {
                Some(f) => !f.is_air(),
                None => false,
            };
            
            let margin = front_t.z - 1. - COLLIDE_PADDING;
            if front && translation.z > margin {
                translation.z = margin;
            }
        }
        
        //
        
        {
            let above_t = Translation::new(translation.x, translation.y.round() + 1., translation.z);
            let above = match self.get_visible_block(&above_t) {
                Some(a) => !a.is_air(),
                None => false,
            };
            
            let margin = above_t.y - 1. - COLLIDE_PADDING;
            if above && translation.y > margin {
                translation.y = margin;
            }
        }
        
        //
        
        {
            let below_t = Translation::new(translation.x, translation.y.round() - 1., translation.z);
            let below = match self.get_visible_block(&below_t) {
                Some(b) => !b.is_air(),
                None => false,
            };
            
            let margin = below_t.y + 1. + COLLIDE_PADDING;
            if below && translation.y < margin {
                translation.y = margin;
            }
            
            //println!("{:?}, {:?}", self.get_visible_block(&below_t), *translation);

        }
        
        //
        
        {
            let left_t = Translation::new(translation.x.round() - 1., translation.y, translation.z);
            let left = match self.get_visible_block(&left_t) {
                Some(l) => !l.is_air(),
                None => false,
            };
            
            let margin = left_t.x + 1. + COLLIDE_PADDING;
            if left && translation.x < margin {
                translation.x = margin;
            }
        }
        
        //
        
        {
            let right_t = Translation::new(translation.x.round() + 1., translation.y, translation.z);
            let right = match self.get_visible_block(&right_t) {
                Some(r) => !r.is_air(),
                None => false,
            };
            
            let margin = right_t.x - 1. - COLLIDE_PADDING;
            if right && translation.x > margin {
                translation.x = margin;
            }
        }
    }
    
    // Get the block at this position in **world** coords.
    // If the sector is generated but not rendered, `None`
    // is returned.
    fn get_visible_block(&self, pos: &Translation) -> Option<&Block> {
        let sector_pos = sector_at(pos);
        
        let pos = (pos.x.round() as i32, pos.y.round() as i32, pos.z.round() as i32);
        
        if let Some(sector) = self.sectors.get(&sector_pos) {
            if sector.model().is_none() && sector.blocks().needs_rendering() {
                return None;
            }
            
            let local = SectorSpaceCoords::new((pos.0 - sector_pos.0 * SECTOR_SIZE as i32) as u8,
                                               (pos.1 - sector_pos.1 * SECTOR_SIZE as i32) as u8,
                                               (pos.2 - sector_pos.2 * SECTOR_SIZE as i32) as u8);
            
            Some(sector.blocks().get(local))
        } else {
            None
        }
    }
    
    fn load_shaders() ->
            Result<(Program<Vertex, (), Uniforms>, Vec<UniformWarning>), ProgramError> {
        
        let (vs, fs) = shader::load_shader_text("vs", "fs");
        
        Program::from_strings(None, &vs, None, &fs)
    }
}

impl<'a> Drop for Terrain<'a> {
    /// This may block for some time, while
    /// the world generation thread stops.
    fn drop(&mut self) {
        self.stop_generator();
    }
}

impl<'a> Drawable for Terrain<'a> {
    //type Vertex = TerrainVertex;
    //type Uniform = TerrainUniforms;
    
    fn draw(&self,
            device: &mut GLFWDevice,
            render_target: &Framebuffer<Flat, Dim2, (), ()>,
            //shader: &Program<Self::Vertex, (), Self::Uniform>,
            camera: &Camera) {
        let frustum = camera.frustum();
        
        device.draw(|| {
            entry(|gpu| {                    
                // TODO: Only bind the texture once, and ensure
                // that the correct one is used.
                pipeline(render_target, CLEAR_COLOR, |shade_gate| {
                    //let mut skipped = 0;
                    //let mut air = 0;
                    
                    for i in &self.sectors {
                        if let Some(model) = i.1.model() {
                            if !sector_visible(&frustum, *i.0) {
                                //skipped += 1;
                                continue;
                            }
                            
                            gpu.bind_texture(&model.tex.0);
                            shade_gate.shade(&self.shader, |render_gate, uniforms| {
                                uniforms.model_matrix.update(model.to_matrix());
                                uniforms.view_matrix.update(camera.to_matrix());
                                uniforms.projection_matrix.update(*camera.projection_matrix());
                                //uniforms.terrain_tex.update(bound);
                                
                                let render_state = RenderState::default();
                                                   //.set_face_culling(None);
                                render_gate.render(render_state, |tess_gate| {
                                    tess_gate.render((&model.tess).into());
                                });
                            });
                        }/* else {
                            air += 1;
                        }*/
                    }
                    
                    //println!("skipped: {} / {})", skipped, self.sectors.len() - air);
                });
            });
        });
    }
}

/// Terrain's uniform interface.
struct Uniforms {
    // Model transform.
    model_matrix: Uniform<M44>,
    
    // Camera view.
    view_matrix: Uniform<M44>,
    
    // 3D Projection.
    projection_matrix: Uniform<M44>,
    
    // Terrain Texture Atlas.
    //pub terrain_tex: Uniform<BoundTexture<'a, Texture<Flat, Dim2, RGB8UI>>>,
}

impl<'a> UniformInterface for Uniforms {
    fn uniform_interface(builder: UniformBuilder)
            -> Result<(Uniforms, Vec<UniformWarning>), ProgramError> {
        
        let model_matrix = builder.ask("model_matrix").unwrap();
        let view_matrix = builder.ask("view_matrix").unwrap();
        let projection_matrix = builder.ask("projection_matrix").unwrap();
        //let terrain_tex = builder.ask("terrain_tex").unwrap();
        
        Ok((Uniforms {
            model_matrix,
            view_matrix,
            projection_matrix,
            //terrain_tex,
        }, Vec::new()))
    }
}

// Information shared between the main thread
// and the worldgen thread.

#[derive(Debug)]
struct WorldGenThreadInfo {
     needed: HashMap<(i32, i32, i32), bool>,
     exiting: bool,
}

type SharedInfo = Arc<Mutex<WorldGenThreadInfo>>;

impl Default for WorldGenThreadInfo {
    fn default() -> WorldGenThreadInfo {
        WorldGenThreadInfo {
            needed: HashMap::new(),
            exiting: false,
        }
    }
}

// The nearest sector at a translation.
fn sector_at(pos: &Translation) -> (i32, i32, i32) {
    ((pos.x.round() / SECTOR_SIZE as f32).floor() as i32,
     (pos.y.round() / SECTOR_SIZE as f32).floor() as i32,
     (pos.z.round() / SECTOR_SIZE as f32).floor() as i32)
}

const SECTOR_SIZE_F: f32 = SECTOR_SIZE as f32;
const SECTOR_SIZE_F_2: f32 = SECTOR_SIZE_F / 2.;

fn sector_visible(frustum: &Frustum, pos: (i32, i32, i32)) -> bool {
    // Convert sector coords to world space.
    let pos = (pos.0 as f32 * SECTOR_SIZE_F + SECTOR_SIZE_F_2,
               pos.1 as f32 * SECTOR_SIZE_F + SECTOR_SIZE_F_2,
               pos.2 as f32 * SECTOR_SIZE_F + SECTOR_SIZE_F_2);
    
    //println!("pos: {:?}", pos);
    //true
    
    for i in frustum.planes() {
        //println!("plane: {:?}", i);
        
        let d = i.a * pos.0 + i.b * pos.1 + i.c * pos.2 + i.d;
        
        if d <= -SECTOR_SIZE_F {
            return false;
        }
    }
    
    true
}
