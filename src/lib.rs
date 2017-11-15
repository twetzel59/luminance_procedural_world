//! `luminance_procedural_world` is an experiement. I intend to
//! create a sandbox block world generator. I do not know wheather
//! this will become a game. Likely, this exact crate will not.
//! The primary purpose is to explore world generation and rendering.

extern crate glfw;
extern crate luminance;
extern crate luminance_glfw;

pub use viewer::Viewer;

pub mod camera;
#[macro_use]
pub mod maths;
pub mod shader;
pub mod viewer;
