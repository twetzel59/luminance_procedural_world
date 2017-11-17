//! This module contains abstractions for drawing tesselations
//! with textures and a camera.

use luminance::linear::M44;
use luminance::pixel::RGB32F;
use luminance::tess::Tess;
use luminance::texture::{Dim2, Flat, Texture};
use luminance::vertex::Vertex;
use maths::{ToMatrix, Translation};

/// Encapsulates a luminance `Tess` and `Texture`, providing
/// a representation of a 3D object.
/// # Generic type parameters
/// * **V**: The type of vertex to use with the tesselation.
pub struct Model<V> {
    /// The model's vertex data.
    pub tess: Tess<V>,
    
    /// The texture for the model.
    pub tex: Texture<Flat, Dim2, RGB32F>,
    
    /// The translation that should
    /// be applied to the model.
    pub translation: Translation,
}

impl<V> Model<V> where V: Vertex {
    /// Create a new model. The model will be centered at the origin.
    pub fn new(tess: Tess<V>, tex: Texture<Flat, Dim2, RGB32F>) -> Model<V> {
        Self::with_translation(tess, tex, Translation::new(0., 0., 0.))
    }
    
    /// Create a new model with the supplied translation.
    pub fn with_translation(tess: Tess<V>, tex: Texture<Flat, Dim2, RGB32F>,
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

/*
/// Typeclass for anything that can render itself with a camera
/// and a framebuffer supplied.
pub trait Drawable {
    fn draw();
}
*/
