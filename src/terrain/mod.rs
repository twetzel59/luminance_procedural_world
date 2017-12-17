//! Module related to managing and drawing terrain.

pub mod mesh_gen;
pub mod voxel;

use std::collections::HashMap;
use luminance::framebuffer::Framebuffer;
use luminance::linear::M44;
use luminance::pipeline::{entry, pipeline, RenderState};
use luminance::texture::{Dim2, Flat};
use luminance::shader::program::{Program, ProgramError, Uniform, UniformBuilder,
                                 UniformInterface, UniformWarning};
use luminance_glfw::{Device, GLFWDevice};
use camera::Camera;
use maths::ToMatrix;
use model::Drawable;
use resources::Resources;
use shader;
use self::voxel::Sector;

// Type of terrain position vertex attribute.
type Position = [f32; 3];

// Type of terrain texture coordinate attribute.
type UV = [f32; 2];

// A terrain vertex.
type Vertex = (Position, UV);

/// Drawable manager for world terrain. Handles the rendering
/// of each sector (**not yet implemented**).
pub struct Terrain {
    sectors: HashMap<(i32, i32, i32), Sector>,
    shader: Program<Vertex, (), Uniforms>,
}

impl Terrain {
    /// Create a new `Terrain` using the shared `Resources`.
    /// # Panics
    /// This constructor panics if shaders fail to load.
    pub fn new(resources: &Resources) -> Terrain {
        let (shader, warnings) = Self::load_shaders().unwrap();
        for warn in &warnings {
            eprintln!("{:?}", warn);
        }
        
        let mut sectors = HashMap::new();
        sectors.insert((0, 0, 0), Sector::new(resources, (0, 0, 0)));
        sectors.insert((1, 0, 0), Sector::new(resources, (1, 0, 0)));
        sectors.insert((0, 0, 1), Sector::new(resources, (0, 0, 1)));
        
        Terrain {
            sectors,
            shader,
        }
    }
    
    fn load_shaders() ->
            Result<(Program<Vertex, (), Uniforms>, Vec<UniformWarning>), ProgramError> {
        
        let (vs, fs) = shader::load_shader_text("vs", "fs");
        
        Program::from_strings(None, &vs, None, &fs)
    }
}

impl Drawable for Terrain {
    //type Vertex = TerrainVertex;
    //type Uniform = TerrainUniforms;
    
    fn draw(&self,
            device: &mut GLFWDevice,
            render_target: &Framebuffer<Flat, Dim2, (), ()>,
            //shader: &Program<Self::Vertex, (), Self::Uniform>,
            camera: &Camera) {
        device.draw(|| {
            entry(|gpu| {                    
                // TODO: Only bind the texture once, and ensure
                // that the correct one is used.
                pipeline(render_target, [0., 0., 0., 1.], |shade_gate| {
                    for i in self.sectors.values() {
                        gpu.bind_texture(&i.model().tex.0);
                        shade_gate.shade(&self.shader, |render_gate, uniforms| {
                            uniforms.model_matrix.update(i.model().to_matrix());
                            uniforms.view_matrix.update(camera.to_matrix());
                            uniforms.projection_matrix.update(*camera.projection_matrix());
                            //uniforms.terrain_tex.update(bound);
                            
                            let render_state = RenderState::default();
                                               //.set_face_culling(None);
                            render_gate.render(render_state, |tess_gate| {
                                tess_gate.render((&i.model().tess).into());
                            });
                        });
                    }
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
