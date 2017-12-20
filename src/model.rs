//! This module contains abstractions for drawing tesselations
//! with textures and a camera.

use std::rc::Rc;
use luminance::framebuffer::Framebuffer;
use luminance::linear::M44;
use luminance::vertex;
use luminance::pixel::RGB32F;
use luminance::tess::Tess;
use luminance::texture::{Dim2, Flat, Texture};
use luminance_glfw::GLFWDevice;
use png::OutputInfo;
use camera::Camera;
use maths::{ToMatrix, Translation};

/// Encapsulates a luminance `Tess` and `Texture`, providing
/// a representation of a 3D object.
/// # Generic type parameters
/// * **V**: The type of vertex to use with the tesselation.

pub struct Model<V> {
    /// The model's vertex data.
    pub tess: Tess<V>,
    
    /// The texture for the model.
    pub tex: Rc<(Texture<Flat, Dim2, RGB32F>, OutputInfo)>,
    
    /// The translation that should
    /// be applied to the model.
    pub translation: Translation,
}

impl<V: vertex::Vertex> Model<V> {
    /// Create a new model. The model will be centered at the origin.
    pub fn new(tess: Tess<V>, tex: Rc<(Texture<Flat, Dim2, RGB32F>, OutputInfo)>) -> Model<V> {
        Self::with_translation(tess, tex, Translation::new(0., 0., 0.))
    }
    
    /// Create a new model with the supplied translation.
    pub fn with_translation(tess: Tess<V>, tex: Rc<(Texture<Flat, Dim2, RGB32F>, OutputInfo)>,
                            translation: Translation) -> Model<V> {
        Model {
            tess,
            tex,
            translation,
        }
    }
}

impl<V> ToMatrix for Model<V> {
    fn to_matrix(&self) -> M44 {
        self.translation.to_matrix()
    }
}

/// Typeclass for anything that can render itself with a device,
/// framebuffer, and camera supplied.
pub trait Drawable {
    /*
    /// The vertex format, used in the type arguments for `shader`
    /// in `draw()`.
    type Vertex;
    
    /// The type of the uniform interface, also used in type arguments
    /// for the `shader` argument of `draw()`.
    type Uniform;
    */
    
    /// Perform the draw call.
    fn draw(&self,
            device: &mut GLFWDevice,
            render_target: &Framebuffer<Flat, Dim2, (), ()>,
            camera: &Camera);
    /*
    fn draw(&self,
            device: &mut GLFWDevice,
            render_target: &Framebuffer<Flat, Dim2, (), ()>,
            shader: &Program<Self::Vertex, (), Self::Uniform>,
            camera: &Camera);
    */
}
