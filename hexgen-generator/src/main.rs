use hexgen_core::game_loop::GameLoop;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
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

    GameLoop::run(240, 0.1, event_loop, window, display,
                  |_g, display|{

                  },
                  |_g| {
                      return;
                  },
                  |_g, display| {
                      Renderer::render(&display);
                  },
                  |_g, e, display, control_flow| {
                      Renderer::window_event_handler(e, display, control_flow);
                  },
    );
}
