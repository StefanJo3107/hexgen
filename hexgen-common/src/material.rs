use crate::shader::Shader;

pub struct Material {
    shader: Shader,
}

impl Material {
    pub fn new(shader: Shader) -> Material {
        Material {
            shader
        }
    }
}