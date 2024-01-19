use egui::{Context, Visuals};
use glium::{Display, Frame};
use glium::glutin::surface::WindowSurface;
use tracing::info;
use winit::window::Window;

pub struct UI {
    width: u8,
    height: u8,
    seed: isize,
    ss_width: u8,
    ss_height: u8
}

impl UI {
    pub fn new() -> UI {
        UI {
            width: 5,
            height: 5,
            seed: 0,
            ss_width: 0,
            ss_height: 0
        }
    }

    pub fn redraw(&mut self, display: &Display<WindowSurface>, window: &Window, egui_glium: &mut egui_glium::EguiGlium, frame: &mut Frame) {
        let repaint_after = egui_glium.run(&window, |egui_ctx| {
            let mut visuals = Visuals::from(Visuals::dark());
            visuals.window_shadow.extrusion = 2.0;
            egui_ctx.set_visuals(visuals);
            self.generation_settings(egui_ctx);
            self.scene_settings(egui_ctx);
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


    fn generation_settings(&mut self, egui_ctx: &Context) {
        egui::Window::new("Generation settings").show(
            egui_ctx,
            |ui| {
                ui.set_width(150.0);
                ui.set_height(100.0);
                ui.vertical(|ui| {
                    ui.add_space(10.0);

                    ui.label("Select width:");
                    ui.add(egui::Slider::new(&mut self.width, 5..=50));

                    ui.label("Select height:");
                    ui.add(egui::Slider::new(&mut self.height, 5..=50));

                    ui.add_space(7.0);
                    ui.horizontal(|ui| {
                        ui.label("Enter seed value: ");
                        ui.add(egui::DragValue::new(&mut self.seed));
                    });
                    ui.add_space(10.0);

                    ui.centered_and_justified(|ui|{
                        if ui.button("Generate").clicked() {
                            self.ss_width = self.width;
                            self.ss_height = self.height;
                        }
                    });
                    ui.add_space(10.0);
                });
            },
        );
    }

    fn scene_settings(&mut self, egui_ctx: &Context) {
        egui::Window::new("Scene settings").show(egui_ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Frame rate:");
                ui.label("23 fps");
            });
            ui.horizontal(|ui| {
                ui.label("Width:");
                if self.ss_width == 0{
                    ui.label("N/A");
                }else {
                    ui.label(self.ss_width.to_string());
                }
            });
            ui.horizontal(|ui| {
                ui.label("Height:");
                if self.ss_height == 0{
                    ui.label("N/A");
                }else {
                    ui.label(self.ss_height.to_string());
                }
            });
        });
    }
}



