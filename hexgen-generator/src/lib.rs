use std::cell::RefCell;
use std::rc::Rc;
use glium::{Display, uniform};
use glium::glutin::surface::WindowSurface;
use noise::{Fbm, NoiseFn, Perlin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use hexgen_common::game_object::GameObject;
use hexgen_common::material::Material;
use hexgen_common::material::shader::Shader;
use hexgen_common::model::Model;
use hexgen_common::transform::{Rotation, Scale, Translation};
use hexgen_common::vector3::Vector3;
use hexgen_renderer::renderer::{Render, Renderer};

pub struct Generator<'a> {
    pub game_objects: Vec<GameObject>,
    pub models: Vec<Rc<RefCell<Model>>>,
    pub renderer: Renderer<'a>,
}

impl<'a> Generator<'a> {
    pub fn new(renderer: Renderer) -> Generator {
        Generator {
            game_objects: vec![],
            models: vec![],
            renderer,
        }
    }

    pub fn init_scene(&mut self, display: &Display<WindowSurface>) {
        let shader = Shader::new("./res/shaders/diffuse.vert", "./res/shaders/diffuse.frag");
        let mut material_water = Material::new(String::from("Water material"), shader, Vector3::new(0.2, 0.0, 0.0), Vector3::new(0.6, 1.0, 1.0), Vector3::one());
        material_water.load_material(display);

        let shader = Shader::new("./res/shaders/diffuse.vert", "./res/shaders/diffuse.frag");
        let mut material_dirt = Material::new(String::from("Dirt material"), shader, Vector3::new(0.2, 0.0, 0.0), Vector3::new(1.0, 0.58, 0.38), Vector3::one());
        material_dirt.load_material(display);

        let shader = Shader::new("./res/shaders/diffuse.vert", "./res/shaders/diffuse.frag");
        let mut material_grass = Material::new(String::from("Grass material"), shader, Vector3::new(0.2, 0.0, 0.0), Vector3::new(0.21, 0.92, 0.68), Vector3::one());
        material_grass.load_material(display);

        let mut water_model = Model::new(String::from("Water model"), material_water);
        water_model.load_model("./res/models/dirt.obj");
        self.models.push(Rc::new(RefCell::new(water_model)));

        let mut dirt_model = Model::new(String::from("Dirt model"), material_dirt);
        dirt_model.load_model("./res/models/dirt.obj");
        self.models.push(Rc::new(RefCell::new(dirt_model)));

        let mut grass_model = Model::new(String::from("Grass model"), material_grass);
        grass_model.load_model("./res/models/dirt.obj");
        self.models.push(Rc::new(RefCell::new(grass_model)));

        let fbm = Fbm::<Perlin>::new(0);
        let noise = PlaneMapBuilder::<_, 2>::new(&fbm).set_size(40, 40)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();

        for z in 0..40 {
            for x in 0..40 {
                let noise_value = noise.get_value(x, z);
                if noise_value < -0.2 {
                    let mut go = GameObject::new(String::from(format!("{} {},{}", "Water", x, z)), self.models[0].clone());
                    go.rotate(Vector3::new(0.0, 0.0, 2.0));
                    go.translate(Vector3::new((x % 2) as f32 / 2.0 + z as f32, -noise_value as f32 / 15.0, x as f32 / 1.2));
                    self.game_objects.push(go);
                } else if noise_value < 0.0 {
                    let mut go = GameObject::new(String::from(format!("{} {},{}", "Dirt", x, z)), self.models[1].clone());
                    go.rotate(Vector3::new(0.0, 0.0, 2.0));
                    go.translate(Vector3::new((x % 2) as f32 / 2.0 + z as f32, -noise_value as f32 / 15.0, x as f32 / 1.2));
                    self.game_objects.push(go);
                } else {
                    let mut go = GameObject::new(String::from(format!("{} {},{}", "Dirt", x, z)), self.models[2].clone());
                    go.rotate(Vector3::new(0.0, 0.0, 2.0));
                    go.translate(Vector3::new((x % 2) as f32 / 2.0 + z as f32, -noise_value as f32 / 15.0, x as f32 / 1.2));
                    self.game_objects.push(go);
                }
            }
        }
        // self.game_objects[0].scale(Vector3::new(1.0, 1.0, 1.0));
    }
}

