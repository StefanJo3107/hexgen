use std::f32::consts::PI;
use glium::Surface;
use hexgen_core::game_loop::GameLoop;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use hexgen_common::vector3::Vector3;
use hexgen_generator::Generator;
use hexgen_generator::ui::{UI};
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

    let event_loop = winit::event_loop::EventLoopBuilder::with_user_event().build();
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
    let init_frame = display.draw();
    window.set_maximized(true);
    window.set_title("Hexgen simulation");
    let perspective = Perspective::new(PI / 3.0, 1024.0, 0.1, init_frame.get_dimensions().0 as f32, init_frame.get_dimensions().1 as f32);
    let camera = Camera::new(Vector3::new(11.0, 7.0, 1.0), Vector3::new(-3.0, -3.0, 0.0), Vector3::up(), perspective);
    let directional_light = DirectionalLight::new(Vector3::new(-0.2, 0.4, -0.7));
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        ..Default::default()
    };
    let renderer = Renderer::new(camera, directional_light, params, Vector3::new(0.02, 0.02, 0.02));
    init_frame.finish().unwrap();
    let generator = Generator::new(renderer);
    let mut ui = UI::new();

    GameLoop::run(generator, 240, 0.1, event_loop, window, display,
                  |g, display| {
                      g.game_state.init_scene(display);
                      info!("Scene initialization finished");
                  },
                  |_g| {
                      return;
                  },
                  move |g, display, egui_glium| {
                      let mut frame = display.draw();
                      g.game_state.renderer.render(&mut g.game_state.game_objects, &mut frame);
                      ui.redraw(&mut g.game_state, g.frame_rate, display, &g.window, egui_glium, &mut frame);
                      frame.finish().unwrap();
                  },
                  |g, e, display, control_flow| {
                      g.game_state.renderer.window_event_handler(e, display, control_flow);
                  },
    );
}