//! The main entry point.

use std::time::Instant;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::{entry, pipeline, RenderState};
use luminance::tess::{Mode, Tess, TessVertices};
use luminance::shader::program::Program;
use luminance_glfw::{Device, Key, WindowDim, WindowOpt, WindowEvent};
use luminance_glfw::*;
use camera::Camera;
use maths::*;
use shader::{self, TerrainUniforms};

const SCREEN_SIZE: (u32, u32) = (800, 800);
const SPEED: f32 = 1.;

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
        
        let pi = ::std::f32::consts::PI;
        let projection_mat = Projection::new(40. * (pi / 180.),
                                             SCREEN_SIZE.0 as f32 / SCREEN_SIZE.1 as f32,
                                             0.1, 100.0).to_matrix();
        let model_mat = Translation::new(-0.2, 0.4, -1.5).to_matrix();
        
        let mut camera = Camera::new();
        
        /*
        let test1 = mat4! [
            1.,  2.,  3.,  4.,
            5.,  6.,  7.,  8.,
            9.,  10., 11., 12.,
            13., 14., 15., 16.,
        ];
        
        let test2 = mat4! [
            17., 18., 19., 20.,
            21., 22., 23., 24.,
            25., 26., 27., 28.,
            29., 30., 31., 32.,
        ];
        
        let test3 = matrix_mul(&test1, &test2);
        
        println!("test3: {:?}", test3);
        */
                
        let mut delta = 0.;
        'app: loop {
            let begin = Instant::now();
            
            for ev in device.events() {
                match ev {
                    WindowEvent::Close | WindowEvent::Key(Key::Escape, _, _, _)
                        => break 'app,
                    _ => {},
                }
            }
            
            match device.lib_handle().get_key(Key::W) {
                Action::Press | Action::Repeat =>
                    camera.translation_mut().slide(0., 0., -SPEED * delta),
                Action::Release => {},
            }
            
            match device.lib_handle().get_key(Key::S) {
                Action::Press | Action::Repeat =>
                    camera.translation_mut().slide(0., 0., SPEED * delta),
                Action::Release => {},
            }
            
            match device.lib_handle().get_key(Key::A) {
                Action::Press | Action::Repeat =>
                    camera.translation_mut().slide(-SPEED * delta, 0., 0.),
                Action::Release => {},
            }
            
            match device.lib_handle().get_key(Key::D) {
                Action::Press | Action::Repeat =>
                    camera.translation_mut().slide(SPEED * delta, 0., 0.),
                Action::Release => {},
            }
            
            match device.lib_handle().get_key(Key::Space) {
                Action::Press | Action::Repeat =>
                    camera.translation_mut().slide(0., SPEED * delta, 0.),
                Action::Release => {},
            }
            
            match device.lib_handle().get_key(Key::LeftShift) {
                Action::Press | Action::Repeat =>
                    camera.translation_mut().slide(0., -SPEED * delta, 0.),
                Action::Release => {},
            }
            
            match device.lib_handle().get_key(Key::Up) {
                Action::Press | Action::Repeat =>
                    camera.rotation_mut().spin(SPEED * delta, 0.),
                Action::Release => {},
            }
            
            match device.lib_handle().get_key(Key::Down) {
                Action::Press | Action::Repeat =>
                    camera.rotation_mut().spin(-SPEED * delta, 0.),
                Action::Release => {},
            }
            
            match device.lib_handle().get_key(Key::Left) {
                Action::Press | Action::Repeat =>
                    camera.rotation_mut().spin(0., SPEED * delta),
                Action::Release => {},
            }
            
            match device.lib_handle().get_key(Key::Right) {
                Action::Press | Action::Repeat =>
                    camera.rotation_mut().spin(0., -SPEED * delta),
                Action::Release => {},
            }
            
            //println!("camera: {:?}", camera.to_matrix());
            println!("camera rotation: {:?}", camera.rotation());
            
            device.draw(|| {
                entry(|_| {
                    pipeline(&screen, [0., 0., 0., 1.], |shade_gate| {
                        shade_gate.shade(&shader, |render_gate, uniforms| {
                            uniforms.model_matrix.update(model_mat);
                            uniforms.view_matrix.update(camera.to_matrix());
                            uniforms.projection_matrix.update(projection_mat);
                            
                            let render_state = RenderState::default()
                                               .set_face_culling(None);
                            render_gate.render(render_state, |tess_gate| {
                                tess_gate.render((&model).into());
                            });
                        });
                    });
                });
            });
            
            let delta_dur = Instant::now() - begin;          
            delta = delta_dur.as_secs() as f32
                    + delta_dur.subsec_nanos() as f32 * 1e-9;
            //println!("delta: {:?}", delta);
        }
    }
}
