//! The main entry point.

use std::time::Instant;
use glfw::CursorMode;
use luminance::framebuffer::Framebuffer;
use luminance::texture::{Dim2, Flat};
use luminance_glfw::{Action, Device, GLFWDevice, GLFWDeviceError, Key,
                     WindowDim, WindowOpt, WindowEvent};
use camera::{Camera, MovementDirection};
use model::Drawable;
use resources::Resources;
use terrain::Terrain;

const SCREEN_SIZE: (u32, u32) = (800, 800);
const SPEED: f32 = 10.;
const SENSITIVITY: f32 = 0.02;

/// The core of the app, manages the program.
pub struct Viewer {
    device: GLFWDevice,
    render_target: Framebuffer<Flat, Dim2, (), ()>,
    resources: Resources,
    camera: Camera,
}

impl Viewer {
    /// Start up!
    pub fn run() {
        let device = Self::create_device().unwrap();
        
        Viewer {
            device,
            render_target: Framebuffer::default([SCREEN_SIZE.0, SCREEN_SIZE.1]),
            resources: Resources::new(),
            camera: Camera::new(SCREEN_SIZE),
        }.start();
    }
    
    fn create_device() -> Result<GLFWDevice, GLFWDeviceError> {
        GLFWDevice::new(WindowDim::Windowed(SCREEN_SIZE.0, SCREEN_SIZE.1),
                        "luminance_basic",
                        WindowOpt::default())
    }
    
    fn start(mut self) {        
        self.device.lib_handle_mut().set_cursor_mode(CursorMode::Disabled);
        
        let terrain = Terrain::new(&self.resources);
        
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
            
            terrain.draw(&mut self.device, &self.render_target, &self.camera);
            
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
}
