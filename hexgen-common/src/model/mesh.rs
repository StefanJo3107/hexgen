use glium::glutin::surface::WindowSurface;
use glium::{Display, IndexBuffer, VertexBuffer};
use russimp::material::PropertyTypeInfo;
use normal::Normal;
use texture::Texture;
use tracing::info;
use vertex::Vertex;
use crate::material::Material;
use crate::material::shader::Shader;
use crate::vector3::Vector3;

pub mod normal;
pub mod texture;
pub mod vertex;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub normals: Vec<Normal>,
    pub indices: Vec<u32>,
    pub textures: Vec<Texture>,
    pub material: Option<Material>,
    pub vertex_buffer: Option<VertexBuffer<Vertex>>,
    pub index_buffer: Option<IndexBuffer<u32>>,
    pub normals_buffer: Option<VertexBuffer<Normal>>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh {
            vertices: vec![],
            normals: vec![],
            indices: vec![],
            textures: vec![],
            material: None,
            vertex_buffer: None,
            index_buffer: None,
            normals_buffer: None,
        }
    }

    pub fn load_mesh(ai_mesh: russimp::mesh::Mesh, materials: &Vec<russimp::material::Material>) -> Mesh {
        let mut mesh = Mesh::new();

        for i in 0..ai_mesh.vertices.len() {
            let position = (
                ai_mesh.vertices[i].x,
                ai_mesh.vertices[i].y,
                ai_mesh.vertices[i].z,
            );
            let mut tex_coords = (0f32, 0f32);
            if let Some(coord) = ai_mesh.texture_coords[0].as_ref() {
                tex_coords = (coord[i].x, coord[i].y);
            }

            let vertex = Vertex::new(position, tex_coords);
            let normal = Normal::new((
                ai_mesh.normals[i].x,
                ai_mesh.normals[i].y,
                ai_mesh.normals[i].z,
            ));

            mesh.add_vertex(vertex);
            mesh.add_normal(normal);
        }

        for face in ai_mesh.faces {
            for index in face.0 {
                mesh.add_index(index);
            }
        }

        let mat = materials.get(ai_mesh.material_index as usize);

        if let Some(mat) = mat {
            let mut mat_name: String = String::from("");
            let mut mat_diffuse: Vector3 = Vector3::zero();
            let mut mat_specular: Vector3 = Vector3::zero();
            let mut mat_ambient: Vector3 = Vector3::zero();

            for prop in &mat.properties {
                if prop.key == "?mat.name" {
                    if let PropertyTypeInfo::String(name) = &prop.data {
                        mat_name = name.clone();
                    }
                } else if prop.key == "$clr.diffuse" {
                    if let PropertyTypeInfo::FloatArray(arr) = &prop.data {
                        mat_diffuse = Vector3::new(arr[0], arr[1], arr[2]);
                    }
                } else if prop.key == "$clr.ambient" {
                    if let PropertyTypeInfo::FloatArray(arr) = &prop.data {
                        mat_ambient = Vector3::new(arr[0], arr[1], arr[2]);
                    }
                } else if prop.key == "$clr.specular" {
                    if let PropertyTypeInfo::FloatArray(arr) = &prop.data {
                        mat_specular = Vector3::new(arr[0], arr[1], arr[2]);
                    }
                }
            }

            let shader = Shader::new("./res/shaders/diffuse.vert", "./res/shaders/diffuse.frag");
            mesh.material = Some(Material::new(mat_name, shader, mat_ambient, mat_diffuse, mat_specular));
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

    pub fn set_vertex_positions_buffer(
        &mut self,
        display: &Display<WindowSurface>,
    ) {
        self.vertex_buffer = Some(VertexBuffer::new(display, &self.vertices).unwrap());
    }

    pub fn set_normals_buffer(&mut self, display: &Display<WindowSurface>) {
        self.normals_buffer = Some(VertexBuffer::new(display, &self.normals).unwrap());
    }

    pub fn set_indices_buffer(&mut self, display: &Display<WindowSurface>) {
        self.index_buffer = Some(IndexBuffer::new(
            display,
            glium::index::PrimitiveType::TrianglesList,
            &self.indices,
        )
            .unwrap());
    }
}
