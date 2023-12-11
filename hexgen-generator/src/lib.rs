use glium::Display;
use glium::glutin::surface::WindowSurface;
use hexgen_common::game_object::GameObject;
use hexgen_common::material::Material;
use hexgen_common::material::shader::Shader;
use hexgen_common::model::Model;
use hexgen_renderer::renderer::Renderer;

pub struct Generator {
    game_objects: Vec<GameObject>,
    renderer: Renderer,
}

impl Generator {
    pub fn new(renderer: Renderer) -> Generator {
        Generator {
            game_objects: vec![],
            renderer,
        }
    }

    pub fn init_scene(display: &Display<WindowSurface>) {
        let shader = Shader::new("./res/shaders/diffuse.vert", "./res/shaders/diffuse.frag");
        let mut material = Material::new(String::from("Diffuse"), shader);
        material.load_material(display);
        let mut model = Model::new(String::from("Teapot model"), material);
        model.load_model("./res/models/teapot.obj");
    }
}

