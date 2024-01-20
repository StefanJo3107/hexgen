pub mod ui;

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
use rand;
use rand::{Rng, SeedableRng};
use tracing::info;

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
        let mut water_model = Model::new(String::from("Water model"));
        water_model.load_model("./res/models/water.obj", display);
        self.models.push(Rc::new(RefCell::new(water_model)));
        let mut water_rocks_model = Model::new(String::from("Water rocks model"));
        water_rocks_model.load_model("./res/models/water_rocks.obj", display);
        self.models.push(Rc::new(RefCell::new(water_rocks_model)));


        let mut dirt_model = Model::new(String::from("Dirt model"));
        dirt_model.load_model("./res/models/dirt.obj", display);
        self.models.push(Rc::new(RefCell::new(dirt_model)));
        let mut dirt_lumber_model = Model::new(String::from("Dirt lumber model"));
        dirt_lumber_model.load_model("./res/models/dirt_lumber.obj", display);
        self.models.push(Rc::new(RefCell::new(dirt_lumber_model)));

        let mut grass_model = Model::new(String::from("Grass model"));
        grass_model.load_model("./res/models/grass.obj", display);
        self.models.push(Rc::new(RefCell::new(grass_model)));
        let mut grass_forest_model = Model::new(String::from("Grass forest model"));
        grass_forest_model.load_model("./res/models/grass_forest.obj", display);
        self.models.push(Rc::new(RefCell::new(grass_forest_model)));
        let mut grass_hill_model = Model::new(String::from("Grass hill model"));
        grass_hill_model.load_model("./res/models/grass_hill.obj", display);
        self.models.push(Rc::new(RefCell::new(grass_hill_model)));

        let mut stone_model = Model::new(String::from("Stone model"));
        stone_model.load_model("./res/models/stone.obj", display);
        self.models.push(Rc::new(RefCell::new(stone_model)));
        let mut stone_hill_model = Model::new(String::from("Stone hill model"));
        stone_hill_model.load_model("./res/models/stone_hill.obj", display);
        self.models.push(Rc::new(RefCell::new(stone_hill_model)));
        let mut stone_mountain_model = Model::new(String::from("Stone mountain model"));
        stone_mountain_model.load_model("./res/models/stone_mountain.obj", display);
        self.models.push(Rc::new(RefCell::new(stone_mountain_model)));

        self.generate_terrain_without_seed(5, 5);
    }

    pub fn generate_terrain_with_seed(&mut self, width: u8, height: u8, seed: u64){
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let fbm = Fbm::<Perlin>::new(rng.gen_range(0, 100));
        let noise = PlaneMapBuilder::<_, 2>::new(&fbm).set_size(40, 40)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();
        self.generate_terrain(rng, noise, width, height);
    }

    pub fn generate_terrain_without_seed(&mut self, width: u8, height: u8){
        let mut rng = rand::rngs::StdRng::from_entropy();
        let fbm = Fbm::<Perlin>::new(rng.gen_range(0, 100));
        let noise = PlaneMapBuilder::<_, 2>::new(&fbm).set_size(40, 40)
            .set_x_bounds(-5.0, 5.0)
            .set_y_bounds(-5.0, 5.0)
            .build();
        self.generate_terrain(rng, noise, width, height);
    }

    fn generate_terrain(&mut self, mut rng: rand::rngs::StdRng, noise: noise::utils::NoiseMap, width: u8, height: u8) {
        self.game_objects.clear();

        for z in 0..height {
            for x in 0..width {
                let noise_value = noise.get_value(x as usize, z as usize);
                // info!("Noise value: {}", noise_value);
                if noise_value < -0.3 {
                    let val = rng.gen_range(0.0, 1.0);
                    let mut go = GameObject::new(String::from(format!("{} {},{}", "Water", x, z)), self.models[if val < 0.75 {0} else {1}].clone());
                    go.translate(Vector3::new((x % 2) as f32 / 2.0 + z as f32, noise_value as f32 / 12.0, x as f32 / 1.2));
                    self.game_objects.push(go);
                } else if noise_value < 0.0 {
                    for i in 0..(1+((1.0+noise_value)*3.0f64).round() as i32) {
                        let mut go = GameObject::new(String::from(format!("{} {},{}", "Dirt", x, z)), self.models[2+rng.gen_range(0,2)].clone());
                        go.translate(Vector3::new((x % 2) as f32 / 2.0 + z as f32, (i as f32)*0.1f32+noise_value as f32 / 12.0, x as f32 / 1.2));
                        self.game_objects.push(go);
                    }
                } else if noise_value < 0.6 {
                    for i in 0..(((1.0+noise_value)*3.0f64).round() as i32) {
                        let mut go = GameObject::new(String::from(format!("{} {},{}", "Dirt", x, z)), self.models[2+rng.gen_range(0,2)].clone());
                        go.translate(Vector3::new((x % 2) as f32 / 2.0 + z as f32, (i as f32)*0.1f32+noise_value as f32 / 12.0, x as f32 / 1.2));
                        self.game_objects.push(go);
                    }
                    let mut go = GameObject::new(String::from(format!("{} {},{}", "Grass", x, z)), self.models[4+rng.gen_range(0,3)].clone());
                    go.translate(Vector3::new((x % 2) as f32 / 2.0 + z as f32, ((((1.0+noise_value)*3.0f64).round() as i32) as f32)*0.1f32+noise_value as f32 / 12.0, x as f32 / 1.2));
                    self.game_objects.push(go);
                } else {
                    for i in 0..(((1.0+noise_value)*3.0f64).round() as i32) {
                        let mut go = GameObject::new(String::from(format!("{} {},{}", "Dirt", x, z)), self.models[2+rng.gen_range(0,2)].clone());
                        go.translate(Vector3::new((x % 2) as f32 / 2.0 + z as f32, (i as f32)*0.1f32+noise_value as f32 / 12.0, x as f32 / 1.2));
                        self.game_objects.push(go);
                    }
                    let mut go = GameObject::new(String::from(format!("{} {},{}", "Stone", x, z)), self.models[7+rng.gen_range(0,3)].clone());
                    go.translate(Vector3::new((x % 2) as f32 / 2.0 + z as f32, ((((1.0+noise_value)*3.0f64).round() as i32) as f32)*0.1f32+noise_value as f32 / 12.0, x as f32 / 1.2));
                    self.game_objects.push(go);
                }
            }
        }
    }
}

