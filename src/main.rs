use std::{
    path::Path,
    thread,
    time::{Duration, Instant},
};

use luminance::{context::GraphicsContext, render_state::RenderState, shader::program::Program};
use luminance_glutin::{
    ElementState, Event, GlutinSurface, KeyboardInput, Surface, VirtualKeyCode, WindowDim,
    WindowEvent, WindowOpt,
};

use ultraviolet::{mat::Mat4, vec::Vec3};

mod rendering;
use rendering::{Semantics, ShaderInterface};
mod game_object;
use game_object::GameObject;
mod input_manager;
use input_manager::InputManager;
mod utils;
use utils::{convert_mat4, load_texture};
mod camera;
use camera::Camera;

const VS: &'static str = include_str!("../assets/shaders/vertex.glsl");
const FS: &'static str = include_str!("../assets/shaders/fragment.glsl");

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 720;

pub enum Tag {
    Player,
}

fn main() {
    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(WIDTH, HEIGHT),
        "Asteroids",
        WindowOpt::default(),
    )
    .expect("Could not create GlutinSurface.");

    let (tex, _tex_width, _tex_height) =
        load_texture(&mut surface, Path::new("assets/images/ship_01.png"));

    let program = Program::<Semantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("Could not create Program.")
        .ignore_warnings();

    let render_st = RenderState::default();

    let back_buffer = surface.back_buffer().unwrap();

    let mut player = GameObject::new(&mut surface, Tag::Player, (0.0, 0.0), 0.075);
    let mut input_manager = InputManager::new();

    // Calculate camera matrices here as the camera won't be moving
    let cam = Camera::default();
    let (view_matrix, proj_matrix) = cam.get_matrices();

    let t_start = Instant::now();
    let mut total_frames: usize = 0;
    let mut last_frametime = Instant::now();

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
                    WindowEvent::KeyboardInput { input, .. } => {
                        let state = match input.state {
                            ElementState::Pressed => true,
                            ElementState::Released => false,
                        };

                        match input.virtual_keycode {
                            Some(VirtualKeyCode::W) => input_manager.up = state,
                            Some(VirtualKeyCode::S) => input_manager.down = state,
                            Some(VirtualKeyCode::A) => input_manager.left = state,
                            Some(VirtualKeyCode::D) => input_manager.right = state,
                            _ => (),
                        };
                    }
                    _ => (),
                }
            }
        }

        // Update State
        let t = t_start.elapsed().as_millis() as f32 / 1000.0;
        let c = t.sin() * 0.05 + 0.15;
        let background_color = [c, c, c, 1.0];

        let frametime = last_frametime.elapsed().as_millis() as f32 / 1000.0;
        last_frametime = Instant::now();

        // Update player
        let mut updated_acc = Vec3::new(0.0, 0.0, 0.0);
        if input_manager.up {
            updated_acc[1] += 1.0
        }
        if input_manager.down {
            updated_acc[1] -= 1.0
        }
        if input_manager.left {
            updated_acc[0] += 1.0
        }
        if input_manager.right {
            updated_acc[0] -= 1.0
        }
        player.update(frametime, updated_acc);

        // Rendering
        surface.pipeline_builder().pipeline(
            &back_buffer,
            background_color,
            |pipeline, mut shd_gate| {
                let bound_tex = pipeline.bind_texture(&tex);

                shd_gate.shade(&program, |iface, mut rdr_gate| {
                    let transform = Mat4::from_translation(player.pos);

                    iface.tex.update(&bound_tex);
                    iface.model.update(convert_mat4(transform));
                    iface.view.update(view_matrix);
                    iface.proj.update(proj_matrix);

                    rdr_gate.render(render_st, |mut tess_gate| {
                        tess_gate.render(&player.quad);
                    });
                });
            },
        );

        surface.swap_buffers();

        // Limit Framerate
        thread::sleep(Duration::new(0, 1000000000 / 60));
        total_frames += 1;
    }

    println!("Total frames rendered: {}", total_frames);
}
