use egui::{Context, Visuals};
use glium::{Display, Frame};
use glium::glutin::surface::WindowSurface;
use tracing::info;
use winit::window::Window;

pub fn redraw(display: &Display<WindowSurface>, window: &Window, egui_glium: &mut egui_glium::EguiGlium, frame: &mut Frame) {
    let repaint_after = egui_glium.run(&window, |egui_ctx| {
        let mut visuals = Visuals::from(Visuals::dark());
        visuals.window_shadow.extrusion = 2.0;
        egui_ctx.set_visuals(visuals);
        example_window(egui_ctx);
    });

    if repaint_after.is_zero() {
        window.request_redraw();
        winit::event_loop::ControlFlow::Poll
    } else if let Some(repaint_after_instant) =
        std::time::Instant::now().checked_add(repaint_after)
    {
        winit::event_loop::ControlFlow::WaitUntil(repaint_after_instant)
    } else {
        winit::event_loop::ControlFlow::Wait
    };

    {
        egui_glium.paint(display, frame);
    }
}

fn example_window(egui_ctx: &Context) {
    egui::Window::new("example window").show(egui_ctx, |ui| {
        ui.visuals_mut().window_shadow.extrusion = 0.0;
        ui.label("Hello World!");
        if ui.button("Quit").clicked() {
            info!("Button clicked");
        }
    });
}