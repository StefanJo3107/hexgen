use glium::implement_vertex;

#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32),
}

implement_vertex!(Normal, normal);

impl Normal {
    pub fn new(normal: (f32, f32, f32)) -> Normal {
        Normal {
            normal
        }
    }
}