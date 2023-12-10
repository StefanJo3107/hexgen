use std::fs::File;
use std::io::Read;

#[derive(Clone)]
pub struct Shader {
    vertex: String,
    fragment: String,
}

impl Shader {
    pub fn new(vertex_filepath: &str, fragment_filepath: &str) -> Shader {
        let mut vertex_file = File::open(vertex_filepath).unwrap_or_else(|_| { panic!("Cannot open vertex shader file: {}", vertex_filepath) });
        let mut vertex_content = String::new();
        vertex_file.read_to_string(&mut vertex_content).unwrap_or_else(|_| { 0 });
        let mut fragment_file = File::open(fragment_filepath).unwrap_or_else(|_| { panic!("Cannot open fragment shader file: {}", fragment_filepath) });
        let mut fragment_content = String::new();
        fragment_file.read_to_string(&mut fragment_content).unwrap_or_else(|_| { 0 });

        Shader {
            vertex: vertex_content,
            fragment: fragment_content,
        }
    }
}