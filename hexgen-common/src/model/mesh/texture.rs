pub struct Texture {
    id: usize,
    tex_type: String,
}

impl Texture {
    pub fn new(id: usize, tex_type: String) -> Texture {
        Texture {
            id,
            tex_type,
        }
    }
}