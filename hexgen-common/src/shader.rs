use std::fs::File;
use std::io::Read;

pub struct Shader {
    source: String,
}

impl Shader {
    pub fn new(filepath: &str) -> Shader {
        let mut shader_file = File::open(filepath)?;
        let mut content = String::new();
        shader_file.read_to_string(&mut content)?;
        Shader {
            source: content
        }
    }
}