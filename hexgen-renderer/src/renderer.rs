use glium::Surface;
use tracing::info;

pub struct Renderer {

}

impl Renderer {
    pub fn run() {
        info!("Window creation started");
        let event_loop = winit::event_loop::EventLoopBuilder::new().build();
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);
        info!("Window creation finished");

        event_loop.run(move |ev, _, control_flow| {
            match ev {
                winit::event::Event::WindowEvent { event, .. } => match event {
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = winit::event_loop::ControlFlow::Exit;
                    }
                    winit::event::WindowEvent::Resized(window_size) => {
                        display.resize(window_size.into());
                    }
                    _ => ()
                },
                winit::event::Event::RedrawEventsCleared => {
                    window.request_redraw();
                }
                winit::event::Event::RedrawRequested(_) => {
                    let mut frame = display.draw();
                    frame.clear_color_and_depth((1.0, 1.0, 1.0, 1.0), 1.0);
                    frame.finish().unwrap();
                }
                _ => ()
            }
        });
    }
}