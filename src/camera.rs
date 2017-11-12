//! The first person camera is in this module.

use luminance::linear::M44;
use maths::{ToMatrix, Translation};

pub struct Camera {
    pos: Translation,
}

impl Camera {
    /// Creates a camera centered at the origin (0, 0, 0).
    pub fn new() -> Camera {
        Camera {
            pos: Translation::new(0., 0., 0.,),
        }
    }
    
    /// Allows reading of the camera's translation.
    pub fn translation(&self) -> &Translation {
        &self.pos
    }
    
    /// Allows access to the camera's translation.
    pub fn translation_mut(&mut self) -> &mut Translation {
        &mut self.pos
    }
}

impl ToMatrix for Camera {
    fn to_matrix(&self) -> M44 {
        self.pos.to_matrix()
    }
}
