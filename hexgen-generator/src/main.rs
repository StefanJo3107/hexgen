use std::f32::consts::PI;
use glium::{Display, Surface};
use glium::glutin::surface::WindowSurface;
use hexgen_core::game_loop::GameLoop;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use hexgen_common::vector3::Vector3;
use hexgen_generator::Generator;
use hexgen_renderer::camera::Camera;
use hexgen_renderer::camera::perspective::Perspective;
use hexgen_renderer::directional_light::DirectionalLight;
use hexgen_renderer::renderer::Renderer;

fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    info!("Initialized tracing logger");
    info!("Running generator");

    let event_loop = winit::event_loop::EventLoopBuilder::new().build();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let init_frame = display.draw();
    let perspective = Perspective::new(PI / 3.0, 1024.0, 0.1, init_frame.get_dimensions().0 as f32, init_frame.get_dimensions().1 as f32);
    let camera = Camera::new(Vector3::new(10.0, 0.0, 4.0), Vector3::new(-6.0, 2.5, 1.5), Vector3::up(), perspective);
    let directional_light = DirectionalLight::new(Vector3::new(0.6, 0.1, -0.7));
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let renderer = Renderer::new(camera, directional_light, params);
    init_frame.finish().unwrap();
    let generator = Generator::new(renderer);

    GameLoop::run(generator,240, 0.1, event_loop, window, display,
                  |g, display| {
                      g.game_state.init_scene(display);
                      info!("Scene initialization finished");
                  },
                  |_g| {
                      return;
                  },
                  |g, display| {
                      g.game_state.renderer.render(&display, &mut g.game_state.game_objects);
                  },
                  |g, e, display, control_flow| {
                      g.game_state.renderer.window_event_handler(e, display, control_flow);
                  },
    );
}
