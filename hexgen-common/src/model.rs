use glium::Display;
use glium::glutin::surface::WindowSurface;
use russimp::scene::{PostProcess, Scene};
use tracing::info;
use crate::material::Material;
use crate::model::mesh::Mesh;

pub mod mesh;

pub struct Model {
    name: String,
    pub meshes: Vec<Mesh>,
}

impl Model {
    pub fn new(name: String) -> Model {
        Model {
            name,
            meshes: vec![],
        }
    }

    pub fn load_model(&mut self, file_path: &str, display: &Display<WindowSurface>) {
        let scene = Scene::from_file(file_path,
                                     vec![PostProcess::CalculateTangentSpace,
                                          PostProcess::Triangulate,
                                          PostProcess::JoinIdenticalVertices,
                                          PostProcess::SortByPrimitiveType]).unwrap_or_else(|_| { panic!("Couldn't load provided model") });

        info!("Loaded scene from filepath {}", file_path);
        for mesh in scene.meshes {
            self.meshes.push(Mesh::load_mesh(mesh, scene.materials.as_ref()));
        }
        info!("Loaded model '{}'", self.name);
        self.load_materials(display);
    }

    fn load_materials(&mut self, display: &Display<WindowSurface>){
        for mesh in &mut self.meshes {
            if let Some(ref mut material) = &mut mesh.material {
                material.load_material(display);
            }
        }
    }
}
