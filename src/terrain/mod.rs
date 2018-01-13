//! Module related to managing, drawing, and colliding with terrain.

mod mesh_gen;
mod voxel;
mod world_gen;

use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use linked_hash_map::LinkedHashMap;
use luminance::framebuffer::Framebuffer;
use luminance::linear::M44;
use luminance::pipeline::{entry, pipeline, RenderState};
use luminance::texture::{Dim2, Flat};
use luminance::shader::program::{Program, ProgramError, Uniform, UniformBuilder,
                                 UniformInterface, UniformWarning};
use luminance_glfw::{Device, GLFWDevice};
use png::OutputInfo;
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

/// The length of one side of a cubic sector, **excluding** padding.
pub const SECTOR_SIZE: usize = 32;

/// A variant of the sector size of the type `u32`.
pub const SECTOR_SIZE_U32: u32 = SECTOR_SIZE as u32;

/// The amount of padding on each side of a cubic side of a sector.
pub const SECTOR_PAD: usize = 1;

/// A variant of the amount of sector padding of the type `u32`.
pub const SECTOR_PAD_U32: u32 = SECTOR_PAD as u32;

/// The length of a sector's side, including padding.
pub const SECTOR_SIZE_PAD: usize = SECTOR_PAD + SECTOR_SIZE + SECTOR_PAD;

/// A variant of the padded sector size of type type `u32`.
pub const SECTOR_SIZE_PAD_U32: u32 = SECTOR_SIZE_PAD as u32;

/// The length of an array of blocks for a sector.
pub const SECTOR_LEN: usize = SECTOR_SIZE_PAD * SECTOR_SIZE_PAD * SECTOR_SIZE_PAD;

const CLEAR_COLOR: [f32; 4] = [0.2, 0.75, 0.8, 1.0];
const COLLIDE_PADDING: f32 = 0.3;
const NUM_THREADS: usize = 8;
const GENERATE_ORDER: [i32; 9] = [0, -1, 1, 2, -2, 3, -3, 4, -4];
const MAX_PENDING_SECTORS: usize = NUM_THREADS * 4;
const MAX_PENDING_REQUESTS: usize = 32;
const MAX_LAG: f64 = 0.05;

/// Drawable manager for world terrain. Handles the rendering
/// of each sector.
pub struct Terrain<'a> {
    shader: Program<Vertex, (), Uniforms>,
    resources: &'a Resources,
    sectors: HashMap<(i32, i32, i32), Sector>,
    shared_info: SharedInfo,
    join_handles: [Option<JoinHandle<()>>; NUM_THREADS],
    generated_tx: SyncSender<Generated>,
    generated_rx: Receiver<Generated>,
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
        
        let (generated_tx, generated_rx) = mpsc::sync_channel(MAX_PENDING_SECTORS);
        
        Terrain {
            resources,
            sectors: HashMap::with_capacity(1000),
            shader,
            shared_info: Arc::new(Mutex::new(Default::default())),
            join_handles: Default::default(),
            generated_tx,
            generated_rx,
        }
    }
    
    /// Spawn the world generation thread.
    /// The terrain will immediately begin generating.
    pub fn spawn_generator(&mut self) {
        for i in self.join_handles.iter_mut() {        
            let shared_info = self.shared_info.clone();
            let generated_tx = self.generated_tx.clone();
            let tex = self.resources.terrain_tex();
            // 3rd party lacks `Clone` impl, but POD
            // struct contents is enough.
            let tex_info = OutputInfo {
                width: tex.1.width,
                height: tex.1.height,
                color_type: tex.1.color_type,
                bit_depth: tex.1.bit_depth,
                line_size: tex.1.line_size,
            };
            
            *i = Some(thread::spawn(move || {
                let wg = WorldGen::new();
                
                loop {
                    let mut shared_info = shared_info.lock().unwrap();
                    
                    if shared_info.exiting {
                        return;
                    }
                    
                    //println!("len: {}", shared_info.needed.len());
                    
                    let mut sector = None;
                    for i in shared_info.needed.iter_mut() {
                        //println!("{:?}", i);
                        if *i.1 {
                            *i.1 = false;
                            //println!("{:?}", i.0);
                            
                            sector = Some(*i.0);
                            break;
                        }
                    }
                    
                    if let Some(s) = sector {
                        shared_info.needed.remove(&s);
                        mem::drop(shared_info);
                        
                        let list = wg.generate(s);
                        let vertices = mesh_gen::generate_block_vertices(&list, &tex_info);
                        
                        let generated = Generated {
                            pos: s,
                            list,
                            vertices,
                        };
                        
                        let _ = generated_tx.send(generated);
                    } else {
                        mem::drop(shared_info);
                    }
                    
                    // ABSOLUTELY CRITICAL
                    // to avoid deadlock
                    thread::sleep(Duration::from_millis(5));
                    //
                }
            }));
        }
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
        println!("Stopping worldgen thread...");
        self.shared_info.lock().unwrap().exiting = true;
        println!("Aquired lock");
        
        while let Ok(_) = self.generated_rx.try_recv() {}
        println!("Drained channel");
        
        for i in self.join_handles.iter_mut().enumerate() {
            if let Some(handle) = i.1.take() {
                handle.join().unwrap();
                println!("Joined worldgen thread {}", i.0);
            }
        }
    }
    
    /// Perform a frame update.
    /// May block for some time until a mutex can be aquired.
    pub fn update(&mut self, camera: &Camera) {
        //self.shared_info.lock().unwrap().player_pos = translation.clone();
        
        let sector = sector_at(&camera.translation());
        
        let mut info = self.shared_info.lock().unwrap();
        if info.needed.len() < MAX_PENDING_REQUESTS {
            for x in &GENERATE_ORDER {
                for y in &GENERATE_ORDER {
                    for z in &GENERATE_ORDER {
                        let new_sector = (sector.0 + x, sector.1 + y, sector.2 + z);
                        if !self.sectors.contains_key(&new_sector) {
                            info.needed.entry(new_sector).or_insert(true);
                        }
                    }
                }
            }
        }
        
        self.sectors.retain(|&k, _| {
            let dx = k.0 as f32 - sector.0 as f32;
            let dy = k.1 as f32 - sector.1 as f32;
            let dz = k.2 as f32 - sector.2 as f32;
            
            let dist_sq = dx * dx + dy * dy + dz * dz;
            
            //println!("{}", dist_sq);
            
            dist_sq < 280.
        });
        
        let begin = Instant::now();
        while let Ok(generated) = self.generated_rx.try_recv() {
            self.sectors.insert(generated.pos,
                                Sector::new(self.resources,
                                            generated.pos,
                                            generated.list,
                                            generated.vertices));
        
            let duration = Instant::now() - begin;
            
            let seconds = duration.as_secs() as f64 +
                          duration.subsec_nanos() as f64 * 1e-9;
            
            if seconds > MAX_LAG {
                //println!("too long: {}", seconds);
                break;
            }
        }
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
            //if sector.model().is_none() && sector.blocks().needs_rendering() {
            //    return None;
            //}
            
            let local = SectorSpaceCoords::new((pos.0 - sector_pos.0 * SECTOR_SIZE as i32) as u32,
                                               (pos.1 - sector_pos.1 * SECTOR_SIZE as i32) as u32,
                                               (pos.2 - sector_pos.2 * SECTOR_SIZE as i32) as u32);
            
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
    needed: LinkedHashMap<(i32, i32, i32), bool>,
    exiting: bool,
}

type SharedInfo = Arc<Mutex<WorldGenThreadInfo>>;

impl Default for WorldGenThreadInfo {
    fn default() -> WorldGenThreadInfo {
        WorldGenThreadInfo {
            needed: LinkedHashMap::new(),
            exiting: false,
        }
    }
}

// A generated `BlockList`
struct Generated {
    pos: (i32, i32, i32),
    list: BlockList,
    vertices: Vec<Vertex>,
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
