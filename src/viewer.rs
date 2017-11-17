//! The main entry point.

use std::fs::File;
use std::time::Instant;
use glfw::CursorMode;
use luminance::framebuffer::Framebuffer;
use luminance::pipeline::{entry, pipeline, RenderState};
use luminance::pixel::RGB32F;
use luminance::tess::{Mode, Tess, TessVertices};
use luminance::texture::{Dim2, Flat, MagFilter, MinFilter, Sampler, Texture};
use luminance::shader::program::{Program, ProgramError, UniformWarning};
use luminance_glfw::{Device, Key, WindowDim, WindowOpt, WindowEvent};
use luminance_glfw::*;
use png::{self, Decoder as PngDecoder};
use camera::{Camera, MovementDirection};
use maths::*;
use model::*;
use shader::{self, TerrainUniforms};

const SCREEN_SIZE: (u32, u32) = (800, 800);
const SPEED: f32 = 1.;
const SENSITIVITY: f32 = 0.02;
const TEXTURE_PATH: &str = "data/tex.png";

type Position = [f32; 3];
type UV = [f32; 2];
type Vertex = (Position, UV);

const VERTICES: [Vertex; 3] = [
  ([-0.5, -0.5, 0.0], [0.0, 1.0]),
  ([-0.5,  0.5, 0.0], [0.0, 0.0]),
  ([ 0.5, -0.5, 0.0], [1.0, 1.0]),
  
];

//const SHADERS: (&str, &str) = (include_str!("../data/vs.glsl"),
//                               include_str!("../data/fs.glsl"));

/// The core of the app, manages the program.
pub struct Viewer {
    device: GLFWDevice,
    shader: Program<Vertex, (), TerrainUniforms>,
    render_target: Framebuffer<Flat, Dim2, (), ()>,
    camera: Camera,
}

impl Viewer {
    /// Start up!
    pub fn run() {
        let device = Self::create_device().unwrap();
        
        let (shader, warnings) = Self::load_shaders().unwrap();
        for warn in &warnings {
            eprintln!("{:?}", warn);
        }
        
        Viewer {
            device,
            shader,
            render_target: Framebuffer::default([SCREEN_SIZE.0, SCREEN_SIZE.1]),
            camera: Camera::new(SCREEN_SIZE),
        }.start();
    }
    
    fn create_device() -> Result<GLFWDevice, GLFWDeviceError> {
        GLFWDevice::new(WindowDim::Windowed(SCREEN_SIZE.0, SCREEN_SIZE.1),
                        "luminance_basic",
                        WindowOpt::default())
    }
    
    fn load_shaders() ->
            Result<(Program<Vertex, (), TerrainUniforms>, Vec<UniformWarning>), ProgramError> {
        
        let (vs, fs) = shader::load_shader_text("vs", "fs");
        
        Program::from_strings(None, &vs, None, &fs)
    }
    
    fn start(mut self) {        
        self.device.lib_handle_mut().set_cursor_mode(CursorMode::Disabled);
        
        let png_decoder = PngDecoder::new(File::open(TEXTURE_PATH).unwrap());
        let (png_info, mut png_reader) = png_decoder.read_info().unwrap();
        assert_eq!(png_info.color_type, png::ColorType::RGB);
        assert_eq!(png_info.bit_depth, png::BitDepth::Eight);
        let mut png_data = vec![0; png_info.buffer_size()];
        png_reader.next_frame(&mut png_data).unwrap();
        
        //println!("size: {:?}", (png_info.width, png_info.height));
        assert_eq!(png_info.buffer_size() % 3, 0);
        let mut image = Vec::with_capacity(png_info.buffer_size() / 3);
        for i in 0..(png_info.buffer_size() / 3) {
            let x = i * 3;
            
            //println!("data: {:?}", &[png_data[x], png_data[x + 1], png_data[x + 2]]);
            image.push((png_data[x]     as f32 / 255.,
                        png_data[x + 1] as f32 / 255.,
                        png_data[x + 2] as f32 / 255.));
        }
        
        let mut sampler = Sampler::default();
        sampler.min_filter = MinFilter::Nearest;
        sampler.mag_filter = MagFilter::Nearest;
        
        let tex = Texture::<Flat, Dim2, RGB32F>::new(
                [png_info.width, png_info.height], 0, &sampler).unwrap();
        tex.upload(false, &image);
        //tex.upload_raw(false, &png_data);
        //tex.clear(false, (128, 0, 128));
        //tex.clear(false, (0.5, 0., 0.5));
        
        let tess = Tess::new(Mode::Triangle, TessVertices::Fill(&VERTICES), None);
        
        //let model = Model::new(tess, tex);
        let model = Model::with_translation(tess, tex, Translation::new(1., 0., -1.));
        
        //let projection_mat = Projection::new(40. * (PI / 180.),
        //                                     SCREEN_SIZE.0 as f32 / SCREEN_SIZE.1 as f32,
        //                                     0.1, 100.0).to_matrix();
        //let model_mat = Translation::new(-0.2, 0.4, -1.5).to_matrix();
        
        //let mut camera = Camera::new(SCREEN_SIZE);
        
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
        loop {
            let begin = Instant::now();
            
            if !self.handle_events() {
                break;
            }
            self.handle_realtime_input(delta);
            
            Self::render(&mut self.device, &self.render_target,
                         &self.shader, &self.camera, &model);
            
            let delta_dur = Instant::now() - begin;          
            delta = delta_dur.as_secs() as f32
                    + delta_dur.subsec_nanos() as f32 * 1e-9;
            //println!("delta: {:?}", delta);
        }
    }
    
    // #Return Value
    // Wheather the game should keep running
    fn handle_events(&mut self) -> bool {
        let mut keep_running = true;
        
        for ev in self.device.events() {
            match ev {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, _, _)
                    => {
                        keep_running = false;
                        break;
                    },
                _ => {},
            }
        }
        
        keep_running
    }
    
    fn handle_realtime_input(&mut self, delta: f32) {
        match self.device.lib_handle().get_key(Key::W) {
            Action::Press | Action::Repeat =>
                self.camera.move_dir(MovementDirection::Forward, SPEED * delta),
            Action::Release => {},
        }
        
        match self.device.lib_handle().get_key(Key::S) {
            Action::Press | Action::Repeat =>
                self.camera.move_dir(MovementDirection::Backward, SPEED * delta),
            Action::Release => {},
        }
        
        match self.device.lib_handle().get_key(Key::A) {
            Action::Press | Action::Repeat =>
                self.camera.move_dir(MovementDirection::Left, SPEED * delta),
            Action::Release => {},
        }
        
        match self.device.lib_handle().get_key(Key::D) {
            Action::Press | Action::Repeat =>
                self.camera.move_dir(MovementDirection::Right, SPEED * delta),
            Action::Release => {},
        }
        
        match self.device.lib_handle().get_key(Key::Space) {
            Action::Press | Action::Repeat =>
                self.camera.translation_mut().slide(0., SPEED * delta, 0.),
            Action::Release => {},
        }
        
        match self.device.lib_handle().get_key(Key::LeftShift) {
            Action::Press | Action::Repeat =>
                self.camera.translation_mut().slide(0., -SPEED * delta, 0.),
            Action::Release => {},
        }
        
        match self.device.lib_handle().get_key(Key::Up) {
            Action::Press | Action::Repeat =>
                self.camera.rotation_mut().spin(SPEED * delta, 0.),
            Action::Release => {},
        }
        
        match self.device.lib_handle().get_key(Key::Down) {
            Action::Press | Action::Repeat =>
                self.camera.rotation_mut().spin(-SPEED * delta, 0.),
            Action::Release => {},
        }
        
        match self.device.lib_handle().get_key(Key::Left) {
            Action::Press | Action::Repeat =>
                self.camera.rotation_mut().spin(0., SPEED * delta),
            Action::Release => {},
        }
        
        match self.device.lib_handle().get_key(Key::Right) {
            Action::Press | Action::Repeat =>
                self.camera.rotation_mut().spin(0., -SPEED * delta),
            Action::Release => {},
        }
        
        //println!("self.camera: {:?}", self.camera.to_matrix());
        //println!("self.camera rotation: {:?}", self.camera.rotation());
        
        //println!("mouse pos: {:?}", self.device.lib_handle().get_cursor_pos());
        let mouse_pos = self.device.lib_handle().get_cursor_pos();
        let mouse_pos = (mouse_pos.0 as f32, mouse_pos.1 as f32);
        self.camera.rotation_mut().spin(SPEED * delta * -mouse_pos.1 * SENSITIVITY,
                                   SPEED * delta * -mouse_pos.0 * SENSITIVITY);
        self.device.lib_handle_mut().set_cursor_pos(0., 0.);
    }
    
    fn render(device: &mut GLFWDevice,
              render_target: &Framebuffer<Flat, Dim2, (), ()>,
              shader: &Program<Vertex, (), TerrainUniforms>,
              camera: &Camera,
              model: &Model<Vertex>) {
        device.draw(|| {
            entry(|gpu| {
                gpu.bind_texture(&model.tex);
                pipeline(render_target, [0., 0., 0., 1.], |shade_gate| {
                    shade_gate.shade(shader, |render_gate, uniforms| {
                        uniforms.model_matrix.update(model.to_matrix());
                        uniforms.view_matrix.update(camera.to_matrix());
                        uniforms.projection_matrix.update(*camera.projection_matrix());
                        //uniforms.terrain_tex.update(bound);
                        
                        let render_state = RenderState::default()
                                           .set_face_culling(None);
                        render_gate.render(render_state, |tess_gate| {
                            tess_gate.render((&model.tess).into());
                        });
                    });
                });
            });
        });
    }
}
