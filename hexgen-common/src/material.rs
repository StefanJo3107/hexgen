use shader::Shader;

pub mod shader;

#[derive(Clone)]
pub struct Material {
    pub shader: Shader,
}

impl Material {
    pub fn new(shader: Shader) -> Material {
        Material {
            shader
        }
    }
}
