use glium::{Display, IndexBuffer, VertexBuffer};
use glium::glutin::surface::WindowSurface;
use tracing::info;
use normal::Normal;
use texture::Texture;
use vertex::Vertex;

pub mod vertex;
pub mod normal;
pub mod texture;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    // material: Material,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: vec![],
            normals: vec![],
            indices: vec![],
            textures: vec![],
            // material,
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

        info!("Loaded mesh");
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

    pub fn get_vertex_positions_buffer(&self, display: &Display<WindowSurface>) -> VertexBuffer<Vertex> {
        VertexBuffer::new(display, &self.vertices).unwrap()
    }

    pub fn get_normals_buffer(&self, display: &Display<WindowSurface>) -> VertexBuffer<Normal> {
        VertexBuffer::new(display, &self.normals).unwrap()
    }

    pub fn get_indices_buffer(&self, display: &Display<WindowSurface>) -> IndexBuffer<u32> {
        IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList, &self.indices).unwrap()
    }
}
