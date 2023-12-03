#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    position: (f32, f32, f32),
    tex_coords: (f32, f32),
}

impl Vertex{
    pub fn new(position: (f32, f32, f32), tex_coords: (f32, f32)) -> Vertex{
        Vertex{
            position, tex_coords
        }
    }
}