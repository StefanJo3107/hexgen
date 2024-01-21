use std::time;
use std::thread;
use std::time::Duration;
use egui_glium::EguiGlium;
use glium::Display;
use glium::glutin::surface::WindowSurface;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub struct GameLoop<G> {
    pub game_state: G,
    pub updates_per_second: usize,
    pub max_frame_time: f64,
    pub window: Window,
    pub window_occluded: bool,
    pub exit_next_iteration: bool,
    fixed_time_step: f64,
    number_of_updates: usize,
    number_of_renders: usize,
    last_frame_time: f64,
    running_time: f64,
    accumulated_time: f64,
    blending_factor: f64,
    previous_instant: time::Instant,
    current_instant: time::Instant,
    pub frame_rate: f64,
}

impl<G: 'static> GameLoop<G> {
    pub fn new(game_state: G, updates_per_second: usize, max_frame_time: f64, window: Window) -> GameLoop<G> {
        GameLoop {
            game_state,
            updates_per_second,
            max_frame_time,
            window,
            window_occluded: false,
            exit_next_iteration: false,
            fixed_time_step: 1.0 / updates_per_second as f64,
            number_of_updates: 0,
            number_of_renders: 0,
            last_frame_time: 0.0,
            running_time: 0.0,
            accumulated_time: 0.0,
            blending_factor: 0.0,
            previous_instant: time::Instant::now(),
            current_instant: time::Instant::now(),
            frame_rate: 0.0
        }
    }

    pub fn run<I, U, R, H>(game_state: G, updates_per_second: usize, max_frame_time: f64, event_loop: EventLoop<()>, window: Window, display: Display<WindowSurface>, mut init: I, mut update: U, mut render: R, mut event_handler: H)
        where I: FnMut(&mut GameLoop<G>, &Display<WindowSurface>) + 'static,
              U: FnMut(&mut GameLoop<G>) + 'static,
              R: FnMut(&mut GameLoop<G>, &Display<WindowSurface>, &mut EguiGlium) + 'static,
              H: FnMut(&mut GameLoop<G>, &Event<()>, &Display<WindowSurface>, &mut ControlFlow) + 'static
    {
        let mut game_loop = GameLoop::new(game_state, updates_per_second, max_frame_time, window);
        init(&mut game_loop, &display);
        let mut egui_glium = EguiGlium::new(&display, &game_loop.window, &event_loop);


        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            event_handler(&mut game_loop, &event, &display, control_flow);

            match event {
                Event::RedrawRequested(_) => {
                    if !game_loop.next_frame(&display, &mut update, &mut render, &mut egui_glium) {
                        *control_flow = ControlFlow::Exit;
                    }
                }
                Event::MainEventsCleared => {
                    game_loop.window.request_redraw();
                }
                Event::WindowEvent { event: WindowEvent::Occluded(occluded), .. } => {
                    game_loop.window_occluded = occluded;
                }
                Event::WindowEvent { event, .. } => {
                    let event_response = egui_glium.on_event(&event);

                    if event_response.repaint {
                        game_loop.window.request_redraw();
                    }
                }
                _ => {}
            }
        })
    }

    pub fn next_frame<U, R>(&mut self, display: &Display<WindowSurface>, mut update: U, mut render: R, egui_glium: &mut EguiGlium) -> bool
        where U: FnMut(&mut GameLoop<G>),
              R: FnMut(&mut GameLoop<G>, &Display<WindowSurface>, &mut EguiGlium)
    {
        if self.exit_next_iteration { return false; }

        self.current_instant = time::Instant::now();

        let mut elapsed = (self.current_instant - self.previous_instant).as_secs_f64();
        if elapsed > self.max_frame_time { elapsed = self.max_frame_time; }

        self.last_frame_time = elapsed;
        self.running_time += elapsed;
        self.accumulated_time += elapsed;

        while self.accumulated_time >= self.fixed_time_step {
            update(self);
            self.accumulated_time -= self.fixed_time_step;
            self.number_of_updates += 1;
        }

        self.blending_factor = self.accumulated_time / self.fixed_time_step;

        if self.window_occluded {
            thread::sleep(Duration::from_secs_f64(self.fixed_time_step));
        } else {
            render(self, display, egui_glium);
            self.number_of_renders += 1;
        }
        let frame_duration = (time::Instant::now() - self.current_instant).as_secs_f64();
        if frame_duration != 0.0 {
            self.frame_rate = 1.0 / frame_duration;
        }
        self.previous_instant = self.current_instant;

        true
    }
}