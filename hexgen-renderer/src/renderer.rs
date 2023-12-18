use glium::{Display, DrawParameters, Frame, Surface, uniform};
use glium::glutin::surface::WindowSurface;
use glium::uniforms::{AsUniformValue, Uniforms, UniformsStorage};
use tracing::{error, info};
use winit::event::Event;
use winit::event_loop::{ControlFlow};
use hexgen_common::game_object::GameObject;
use crate::camera::Camera;
use crate::directional_light::DirectionalLight;

pub struct Renderer<'a> {
    pub camera: Camera,
    pub directional_light: DirectionalLight,
    pub draw_parameters: DrawParameters<'a>,
}

impl<'a> Renderer<'a> {
    pub fn new(camera: Camera, directional_light: DirectionalLight, draw_parameters: DrawParameters) -> Renderer {
        Renderer {
            camera,
            directional_light,
            draw_parameters,
        }
    }
    pub fn init() {
        // let model =
    }

    pub fn render(&mut self, display: &Display<WindowSurface>, mut game_objects: &mut Vec<GameObject>) {
        let mut frame = display.draw();
        frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
        let light_dir: [f32; 3] = self.directional_light.direction.into();
        for mut go in game_objects.iter_mut() {
            let model_rc = go.model.clone();
            let model = model_rc.borrow();
            let ambient_color: [f32;3] = model.material.ambient_color.into();
            let diffuse_color: [f32;3] = model.material.diffuse_color.into();
            let specular_color: [f32;3] = model.material.specular_color.into();
            let uniforms = uniform! {
                    //vertex uniforms
                    model: go.model_matrix.0.clone(),
                    perspective: self.camera.perspective.perspective_matrix.0.clone(),
                    view: self.camera.view_matrix.0.clone(),

                    //fragment uniforms
                    u_light: light_dir,
                    ambient_color: ambient_color,
                    diffuse_color: diffuse_color,
                    specular_color: specular_color,
                };

            go.render(&display, &mut frame, uniforms, &self.draw_parameters, (1.0, 1.0, 1.0, 1.0));
        }
        frame.finish().unwrap();
    }
    pub fn window_event_handler(&mut self, event: &Event<()>, display: &Display<WindowSurface>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    info!("Window close event triggered");
                    *control_flow = ControlFlow::Exit;
                }
                winit::event::WindowEvent::Resized(window_size) => {
                    info!("Window resized event triggered");
                    display.resize((window_size.width, window_size.height));
                    self.camera.perspective.update(window_size.width as f32, window_size.height as f32);
                }
                _ => ()
            },
            _ => ()
        }
    }
}

pub trait Render {
    fn render<T, R>(&mut self, display: &Display<WindowSurface>, frame: &mut Frame, uniforms: UniformsStorage<T, R>, draw_parameters: &DrawParameters, background_color: (f32, f32, f32, f32)) where T: AsUniformValue, R: Uniforms;
}

impl<'a> Render for GameObject {
    fn render<T, R>(&mut self, display: &Display<WindowSurface>, frame: &mut Frame, uniforms: UniformsStorage<T, R>, draw_parameters: &DrawParameters, background_color: (f32, f32, f32, f32)) where T: AsUniformValue, R: Uniforms {
        let model_rc = self.model.clone();
        let model = model_rc.borrow();
        for mesh in model.meshes.iter() {
            if let Some(program) = &model.material.program {
                frame.draw((&mesh.get_vertex_positions_buffer(display), &mesh.get_normals_buffer(display)), &mesh.get_indices_buffer(display), program, &uniforms, draw_parameters).unwrap();
            } else {
                error!("Glium program is not initialized");
                break;
            }
        }
    }
}