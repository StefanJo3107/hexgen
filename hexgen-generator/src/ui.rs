use egui::{Context, Key, ScrollArea, Visuals};
use glium::{Display, Frame};
use glium::glutin::surface::WindowSurface;
use tracing::info;
use winit::window::Window;
use crate::Generator;

pub struct UI {
    width: u8,
    height: u8,
    seed: u64,
    define_seed: bool
}

impl UI {
    pub fn new() -> UI {
        UI {
            width: 5,
            height: 5,
            seed: 0,
            define_seed: false
        }
    }

    pub fn redraw(&mut self, generator: &mut Generator, frame_rate: f64, display: &Display<WindowSurface>, window: &Window, egui_glium: &mut egui_glium::EguiGlium, frame: &mut Frame) {
        let repaint_after = egui_glium.run(&window, |egui_ctx| {
            let mut visuals = Visuals::from(Visuals::dark());
            visuals.window_shadow.extrusion = 2.0;
            egui_ctx.set_visuals(visuals);
            self.initialize_shortcuts(egui_ctx, generator);
            self.generation_settings(generator, egui_ctx);
            self.scene_settings(generator, frame_rate, egui_ctx);
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

    fn initialize_shortcuts(&mut self, egui_ctx: &Context, generator: &mut Generator){
        if egui_ctx.input(|i| i.key_pressed(Key::D)) {
            generator.renderer.camera.position.z += 0.5;
            generator.renderer.camera.recalculate();
        }
        if egui_ctx.input(|i| i.key_pressed(Key::A)) {
            generator.renderer.camera.position.z -= 0.5;
            generator.renderer.camera.recalculate();
        }
        if egui_ctx.input(|i| i.key_pressed(Key::S)) {
            generator.renderer.camera.position.x += 0.5;
            generator.renderer.camera.recalculate();
        }
        if egui_ctx.input(|i| i.key_pressed(Key::W)) {
            generator.renderer.camera.position.x -= 0.5;
            generator.renderer.camera.recalculate();
        }
        if egui_ctx.input(|i| i.key_pressed(Key::E)) {
            generator.renderer.camera.position.y += 0.5;
            generator.renderer.camera.recalculate();
        }
        if egui_ctx.input(|i| i.key_pressed(Key::Q)) {
            generator.renderer.camera.position.y -= 0.5;
            generator.renderer.camera.recalculate();
        }
        if let scroll_delta= egui_ctx.input(|i| i.scroll_delta) {
            let sensitivity = 0.01;
            if egui_ctx.input(|i| i.key_pressed(Key::Z)) {
                generator.renderer.camera.direction.z += scroll_delta.y * sensitivity;
                generator.renderer.camera.recalculate();
            }
            if egui_ctx.input(|i| i.key_pressed(Key::X)) {
                generator.renderer.camera.direction.x += scroll_delta.y * sensitivity;
                generator.renderer.camera.recalculate();
            }

        }
    }

    fn generation_settings(&mut self, generator: &mut Generator, egui_ctx: &Context) {
        egui::Window::new("Generation").show(
            egui_ctx,
            |ui| {
                ui.set_width(150.0);
                ui.set_height(100.0);
                ui.vertical(|ui| {
                    ui.add_space(10.0);

                    ui.label("Select width:");
                    ui.add(egui::Slider::new(&mut self.width, 3..=50));

                    ui.label("Select height:");
                    ui.add(egui::Slider::new(&mut self.height, 3..=50));

                    ui.add_space(7.0);
                    ui.checkbox(&mut self.define_seed, "Define seed");
                    if self.define_seed {
                        ui.horizontal(|ui| {
                            ui.label("Enter seed value: ");
                            ui.add(egui::DragValue::new(&mut self.seed));
                        });
                    }
                    ui.add_space(10.0);


                    ui.centered_and_justified(|ui|{
                        if ui.button("Generate").clicked() {
                            if self.define_seed {
                                generator.generate_terrain_with_seed(self.width, self.height, self.seed);
                            } else{
                                generator.generate_terrain_without_seed(self.width, self.height);
                            }
                        }
                    });
                    ui.add_space(10.0);
                });
            },
        );
    }

    fn scene_settings(&mut self, generator: &mut Generator, frame_rate: f64, egui_ctx: &Context) {
        egui::Window::new("Scene").show(egui_ctx, |ui| {
            ui.label("Camera position:");
            ui.horizontal(|ui|{
                let response_x =  ui.add(egui::DragValue::new(&mut generator.renderer.camera.position.x));
                let response_y = ui.add(egui::DragValue::new(&mut generator.renderer.camera.position.y));
                let response_z = ui.add(egui::DragValue::new(&mut generator.renderer.camera.position.z));

                if response_x.changed() || response_y.changed() || response_z.changed() {
                    generator.renderer.camera.recalculate();
                }
            });

            ui.label("Camera direction:");
            ui.horizontal(|ui|{
                let response_x = ui.add(egui::DragValue::new(&mut generator.renderer.camera.direction.x));
                let response_y = ui.add(egui::DragValue::new(&mut generator.renderer.camera.direction.y));
                let response_z = ui.add(egui::DragValue::new(&mut generator.renderer.camera.direction.z));

                if response_x.changed() || response_y.changed() || response_z.changed() {
                    generator.renderer.camera.recalculate();
                }
            });

            ui.label("Light direction:");
            ui.horizontal(|ui|{
                ui.add(egui::DragValue::new(&mut generator.renderer.directional_light.direction.x));
                ui.add(egui::DragValue::new(&mut generator.renderer.directional_light.direction.y));
                ui.add(egui::DragValue::new(&mut generator.renderer.directional_light.direction.z));
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Frame rate:");
                ui.label(format!("{:.2} fps", frame_rate));
            });
            ui.horizontal(|ui| {
                ui.label("Width:");
                ui.label(self.width.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Height:");
                ui.label(self.height.to_string());
            });
        });
    }
}