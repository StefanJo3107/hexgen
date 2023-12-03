use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use hexgen_renderer::renderer::Renderer;

fn main() {
    //tracing init
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    info!("Initialized tracing logger");
    info!("Running renderer");
    Renderer::run();
}
