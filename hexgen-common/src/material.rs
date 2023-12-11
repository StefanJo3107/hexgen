use glium::{Display, Program};
use glium::glutin::surface::WindowSurface;
use tracing::info;
use shader::Shader;

pub mod shader;

pub struct Material {
    pub name: String,
    pub shader: Shader,
    pub program: Option<Program>,
}

impl Material {
    pub fn new(name: String, shader: Shader) -> Material {
        Material {
            name,
            shader,
            program: None,
        }
    }

    pub fn load_material(&mut self, display: &Display<WindowSurface>) {
        self.program = Some(glium::Program::from_source(display, &self.shader.vertex, &self.shader.fragment, None).unwrap());
        info!("Loaded material '{}'", self.name);
    }
}
