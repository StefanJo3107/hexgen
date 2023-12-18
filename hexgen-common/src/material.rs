use glium::{Display, Program};
use glium::glutin::surface::WindowSurface;
use tracing::info;
use shader::Shader;
use crate::vector3::Vector3;

pub mod shader;

pub struct Material {
    pub name: String,
    pub shader: Shader,
    pub program: Option<Program>,
    pub ambient_color: Vector3,
    pub diffuse_color: Vector3,
    pub specular_color: Vector3,
}

impl Material {
    pub fn new(name: String, shader: Shader, ambient_color: Vector3, diffuse_color: Vector3, specular_color: Vector3) -> Material {
        Material {
            name,
            shader,
            program: None,
            ambient_color,
            diffuse_color,
            specular_color,
        }
    }

    pub fn load_material(&mut self, display: &Display<WindowSurface>) {
        self.program = Some(glium::Program::from_source(display, &self.shader.vertex, &self.shader.fragment, None).unwrap());
        info!("Loaded material '{}'", self.name);
    }
}
