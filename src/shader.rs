//! Shader uniform interfaces and utilities.

use std::fs::File;
use std::io::Read;
use std::path::MAIN_SEPARATOR;
use luminance::linear::M44;
use luminance::shader::program::{ProgramError, Uniform, UniformBuilder,
                                 UniformInterface, UniformWarning};

const SHADER_DIR: &str = "shaders";
const EXTENTION: &str = ".glsl";

/// Load shader source from shader names.
/// **Note:** the arguments take the filename, not the path.
/// do not include the full path in the arguments.
pub fn load_shader_text(vertex: &str, fragment: &str) -> (String, String) {
    let mut vs = String::new();
    let mut fs = String::new();
    
    let mut dir = SHADER_DIR.to_string();
    dir.push(MAIN_SEPARATOR);
    
    File::open(dir.clone() + vertex + EXTENTION).unwrap()
        .read_to_string(&mut vs).unwrap();
        
    File::open(dir + fragment + EXTENTION).unwrap()
        .read_to_string(&mut fs).unwrap();
    
    (vs, fs)
}

/// The uniform interface.
pub struct TerrainUniforms {
    /// Model transform.
    pub model_matrix: Uniform<M44>,
}

impl UniformInterface for TerrainUniforms {
    fn uniform_interface(builder: UniformBuilder)
            -> Result<(TerrainUniforms, Vec<UniformWarning>), ProgramError> {
        
        let model_matrix = builder.ask("model_matrix").unwrap();
        
        Ok((TerrainUniforms {
            model_matrix,
        }, Vec::new()))
    }
}
