use russimp::scene::{PostProcess, Scene};
use tracing::info;
use crate::material::Material;
use crate::model::mesh::Mesh;

pub mod mesh;

pub struct Model {
    name: String,
    pub meshes: Vec<Mesh>,
    pub material: Material,
}

impl Model {
    pub fn new(name: String, material: Material) -> Model {
        Model {
            name,
            meshes: vec![],
            material,
        }
    }

    pub fn load_model(&mut self, file_path: &str) {
        let scene = Scene::from_file(file_path,
                                     vec![PostProcess::CalculateTangentSpace,
                                          PostProcess::Triangulate,
                                          PostProcess::JoinIdenticalVertices,
                                          PostProcess::SortByPrimitiveType]).unwrap_or_else(|_| { panic!("Couldn't load provided model") });

        info!("Loaded scene from filepath {}", file_path);
        for mesh in scene.meshes {
            self.meshes.push(Mesh::load_mesh(mesh));
        }
        info!("Loaded model '{}'", self.name);
    }
}
