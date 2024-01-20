use crate::camera::Camera;
use crate::directional_light::DirectionalLight;
use glium::glutin::surface::WindowSurface;
use glium::uniforms::{AsUniformValue, Uniforms, UniformsStorage};
use glium::{uniform, Display, DrawParameters, Frame, Surface};
use hexgen_common::game_object::GameObject;
use tracing::{error, info};
use winit::event::Event;
use winit::event_loop::ControlFlow;
use hexgen_common::matrix::Matrix;
use hexgen_common::vector3::Vector3;

pub struct Renderer<'a> {
    pub camera: Camera,
    pub directional_light: DirectionalLight,
    pub draw_parameters: DrawParameters<'a>,
    pub background_color: Vector3
}

impl<'a> Renderer<'a> {
    pub fn new(
        camera: Camera,
        directional_light: DirectionalLight,
        draw_parameters: DrawParameters,
        background_color: Vector3
    ) -> Renderer {
        Renderer {
            camera,
            directional_light,
            draw_parameters,
            background_color
        }
    }

    pub fn render(
        &mut self,
        display: &Display<WindowSurface>,
        mut game_objects: &mut Vec<GameObject>,
        mut frame: &mut Frame,
    ) {
        frame.clear_color_and_depth((self.background_color.x, self.background_color.y, self.background_color.z, 1.0), 1.0);
        let light_dir: [f32; 3] = self.directional_light.direction.into();
        for mut go in game_objects.iter_mut() {
            let model_rc = go.model.clone();
            let model = model_rc.borrow();

            go.render(
                go.model_matrix.0.clone(),
                self.camera.perspective.perspective_matrix.0.clone(),
                self.camera.view_matrix.0.clone(),
                light_dir,
                &display,
                &mut frame,
                &self.draw_parameters,
            );
        }
    }

    pub fn window_event_handler(
        &mut self,
        event: &Event<()>,
        display: &Display<WindowSurface>,
        control_flow: &mut ControlFlow,
    ) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    info!("Window close event triggered");
                    *control_flow = ControlFlow::Exit;
                }
                winit::event::WindowEvent::Resized(window_size) => {
                    info!("Window resized event triggered");
                    display.resize((window_size.width, window_size.height));
                    self.camera
                        .perspective
                        .update(window_size.width as f32, window_size.height as f32);
                }
                _ => (),
            },
            _ => (),
        }
    }
}

pub trait Render {
    fn render(
        &mut self,
        model_matrix: [[f32; 4]; 4],
        perspective_matrix: [[f32; 4]; 4],
        view_matrix: [[f32; 4]; 4],
        light_dir: [f32; 3],
        display: &Display<WindowSurface>,
        frame: &mut Frame,
        draw_parameters: &DrawParameters,
    );
}

impl<'a> Render for GameObject {
    fn render(
        &mut self,
        model_matrix: [[f32; 4]; 4],
        perspective_matrix: [[f32; 4]; 4],
        view_matrix: [[f32; 4]; 4],
        light_dir: [f32; 3],
        display: &Display<WindowSurface>,
        frame: &mut Frame,
        draw_parameters: &DrawParameters,
    )
    {
        let model_rc = self.model.clone();
        let model = model_rc.borrow();
        for mesh in model.meshes.iter() {
            if let Some(material) = &mesh.material {
                if let Some(program) = &material.program {
                    let ambient_color: [f32; 3] = material.ambient_color.into();
                    let diffuse_color: [f32; 3] = material.diffuse_color.into();
                    let specular_color: [f32; 3] = material.specular_color.into();
                    let uniforms = uniform! {
                //vertex uniforms
                model: model_matrix,
                perspective: perspective_matrix,
                view: view_matrix,

                //fragment uniforms
                u_light: light_dir,
                ambient_color: ambient_color,
                diffuse_color: diffuse_color,
                specular_color: specular_color,
            };
                    frame
                        .draw(
                            (
                                &mesh.get_vertex_positions_buffer(display),
                                &mesh.get_normals_buffer(display),
                            ),
                            &mesh.get_indices_buffer(display),
                            program,
                            &uniforms,
                            draw_parameters,
                        )
                        .unwrap();
                } else {
                    error!("Glium program is not initialized");
                    break;
                }
            }
        }
    }
}

