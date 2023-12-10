use glium::implement_vertex;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: (f32, f32, f32),
    tex_coords: (f32, f32),
}

implement_vertex!(Vertex, position, tex_coords);

impl Vertex {
    pub fn new(position: (f32, f32, f32), tex_coords: (f32, f32)) -> Vertex {
        Vertex {
            position,
            tex_coords,
        }
    }
}