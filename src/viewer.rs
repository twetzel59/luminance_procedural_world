//! The main entry point.

use luminance::framebuffer::Framebuffer;
use luminance::pipeline::{entry, pipeline};
use luminance::tess::{Mode, Tess, TessVertices};
use luminance::shader::program::Program;
use luminance_glfw::{Device, Key, WindowDim, WindowOpt, WindowEvent};
use luminance_glfw::*;
use maths::*;
use shader::{self, TerrainUniforms};

const SCREEN_SIZE: (u32, u32) = (500, 500);

type Position = [f32; 3];
type Color = [f32; 3];
type Vertex = (Position, Color);

const VERTICES: [Vertex; 3] = [
  ([-0.5, -0.5, 0.0], [0.8, 0.5, 0.5]),
  ([-0.5, 0.5, 0.0], [0.5, 0.8, 0.5]),
  ([0.5, -0.5, 0.0], [0.5, 0.5, 0.8]),
  
];

//const SHADERS: (&str, &str) = (include_str!("../data/vs.glsl"),
//                               include_str!("../data/fs.glsl"));

/// The core of the app, manages the program.
pub struct Viewer;

impl Viewer {
    /// Start up!
    pub fn run() {
        let mut device = GLFWDevice::new(
                            WindowDim::Windowed(SCREEN_SIZE.0, SCREEN_SIZE.1),
                            "luminance_basic",
                            WindowOpt::default()).unwrap();
                            
        let model = Tess::new(Mode::Triangle, TessVertices::Fill(&VERTICES), None);
        
        let (vs, fs) = shader::load_shader_text("vs", "fs");
        
        let (shader, warnings) = Program::<Vertex, (), TerrainUniforms>
            ::from_strings(None, &vs, None, &fs).unwrap();
        
        for warn in &warnings {
            eprintln!("{:?}", warn);
        }
        
        let screen = Framebuffer::default([SCREEN_SIZE.0, SCREEN_SIZE.1]);
        
        let matrix = Translation::new(-0.2, 0.4, 0.0).to_matrix();
        
        'app: loop {
            for ev in device.events() {
                match ev {
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, _, _)
                        => break 'app,
                    _ => {},
                }
            }
            
            device.draw(|| {
                entry(|_| {
                    pipeline(&screen, [0., 0., 0., 1.], |shade_gate| {
                        shade_gate.shade(&shader, |render_gate, uniforms| {
                            uniforms.model_matrix.update(matrix);
                            
                            render_gate.render(None, true, |tess_gate| {
                                //unsafe { gl::Enable(gl::CULL_FACE); }
                                
                                tess_gate.render((&model).into());
                                //unsafe { gl::Disable(gl::CULL_FACE); }
                                
                            });
                        });
                    });
                });
            });
        }
    }
}
