use russimp::scene::{PostProcess, Scene};
use crate::mesh::Mesh;

pub struct Model{
    meshes: Vec<Mesh>,
}

impl Model{
    pub fn new() -> Model{
        Model{
            meshes: vec![]
        }
    }

    pub fn load_model(&mut self, file_path: &str) {
        let scene = Scene::from_file(file_path,
                                     vec![PostProcess::CalculateTangentSpace,
                                          PostProcess::Triangulate,
                                          PostProcess::JoinIdenticalVertices,
                                          PostProcess::SortByPrimitiveType]).unwrap_or_else(||{panic!("Couldn't load provided model")});

        for mesh in scene.meshes {
            self.meshes.push(Mesh::load_mesh(mesh));
        }
    }
}
