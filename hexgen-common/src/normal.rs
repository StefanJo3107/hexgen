#[derive(Copy, Clone)]
pub struct Normal {
    normal: (f32, f32, f32),
}

impl Normal {
    pub fn new(normal: (f32, f32, f32)) -> Normal {
        Normal {
            normal
        }
    }
}