use std::num::FpCategory::Normal;
use crate::normal::Normal;
use crate::texture::Texture;
use crate::vertex::Vertex;

pub struct Mesh {
    vertices: Vec<Vertex>,
    normals: Vec<Normal>,
    indices: Vec<u32>,
    textures: Vec<Texture>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: vec![],
            normals: vec![],
            indices: vec![],
            textures: vec![],
        }
    }

    pub fn load_mesh(ai_mesh: russimp::mesh::Mesh) -> Mesh {
        let mut mesh = Mesh::new();

        for i in 0..ai_mesh.vertices.len() {
            let position = (ai_mesh.vertices[i].x, ai_mesh.vertices[i].y, ai_mesh.vertices[i].z);
            let mut tex_coords = (0f32, 0f32);
            if let Some(coord) = ai_mesh.texture_coords[0].as_ref() {
                tex_coords = (coord[i].x, coord[i].y);
            }

            let vertex = Vertex::new(position, tex_coords);
            let normal = Normal::new((ai_mesh.normals[i].x, ai_mesh.normals[i].y, ai_mesh.normals[i].z));

            mesh.add_vertex(vertex);
            mesh.add_normal(normal);
        }

        for face in ai_mesh.faces {
            for index in face.0 {
                mesh.add_index(index);
            }
        }

        return mesh;
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    pub fn add_normal(&mut self, normal: Normal) {
        self.normals.push(normal);
    }

    pub fn add_index(&mut self, index: u32) {
        self.indices.push(index);
    }

    pub fn add_texture(&mut self, texture: Texture) {
        self.textures.push(texture);
    }
}