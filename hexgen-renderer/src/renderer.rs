use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use tracing::info;
use winit::event::Event;
use winit::event_loop::{ControlFlow};
use hexgen_common::game_object::GameObject;
use crate::camera::Camera;
use crate::directional_light::DirectionalLight;

pub struct Renderer {
    camera: Camera,
    directional_light: DirectionalLight,
}

impl Renderer {
    pub fn new(camera: Camera, directional_light: DirectionalLight) -> Renderer {
        Renderer {
            camera,
            directional_light,
        }
    }
    pub fn init() {
        // let model =
    }

    pub fn render(display: &Display<WindowSurface>) {
        let mut frame = display.draw();
        frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
        frame.finish().unwrap();
    }
    pub fn window_event_handler(event: &Event<()>, display: &Display<WindowSurface>, control_flow: &mut ControlFlow) {
        match event {
            Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    info!("Window close event triggered");
                    *control_flow = ControlFlow::Exit;
                }
                winit::event::WindowEvent::Resized(window_size) => {
                    info!("Window resized event triggered");
                    display.resize((window_size.width, window_size.height));
                }
                _ => ()
            },
            _ => ()
        }
    }
}

pub trait Render {
    fn render(display: &Display<WindowSurface>);
}

impl Render for GameObject {
    fn render(display: &Display<WindowSurface>) {
        todo!()
    }
}