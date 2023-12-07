use glium::implement_vertex;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    pub fn new(position: [f32; 3], tex_coords: [f32; 2]) -> Vertex {
        Vertex {
            position,
            tex_coords,
        }
    }
}