use std::{
    thread,
    time::{Duration, Instant},
};

use luminance::context::GraphicsContext;
use luminance_glutin::{
    ElementState, Event, GlutinSurface, KeyboardInput, Surface, VirtualKeyCode, WindowDim,
    WindowEvent, WindowOpt,
};

fn main() {
    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(1280, 720),
        "Asteroids",
        WindowOpt::default(),
    )
    .expect("Could not create GlutinSurface.");

    let back_buffer = surface.back_buffer().unwrap();

    let t_start = Instant::now();
    let mut total_frames: usize = 0;

    'app: loop {
        // Handle Input
        for event in surface.poll_events() {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::Destroyed
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Released,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => break 'app,
                    _ => (),
                }
            }
        }

        // Update State
        let t = t_start.elapsed().as_millis() as f32 / 1000.0;
        let c = t.sin() * 0.05 + 0.15;
        let background_color = [c, c, c, 1.0];

        // Rendering
        surface
            .pipeline_builder()
            .pipeline(&back_buffer, background_color, |_, _| {});

        surface.swap_buffers();

        // Limit Framerate
        thread::sleep(Duration::new(0, 1000000000 / 60));
        total_frames += 1;
    }

    println!("Total frames rendered: {}", total_frames);
}
