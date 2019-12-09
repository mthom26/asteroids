use std::{
    path::Path,
    thread,
    time::{Duration, Instant},
};

use image;

use luminance::{
    context::GraphicsContext,
    linear::M44,
    pipeline::BoundTexture,
    pixel::{NormRGBA8UI, NormUnsigned},
    render_state::RenderState,
    shader::program::{Program, Uniform},
    tess::{Mode, Tess, TessBuilder},
    texture::{Dim2, Flat, GenMipmaps, Sampler, Texture},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glutin::{
    ElementState, Event, GlutinSurface, KeyboardInput, Surface, VirtualKeyCode, WindowDim,
    WindowEvent, WindowOpt,
};

use cgmath::{InnerSpace, Matrix4, Vector3};

const VS: &'static str = include_str!("../assets/shaders/vertex.glsl");
const FS: &'static str = include_str!("../assets/shaders/fragment.glsl");

#[derive(UniformInterface)]
struct ShaderInterface {
    // the 'static lifetime acts as “anything” here
    tex: Uniform<&'static BoundTexture<'static, Flat, Dim2, NormUnsigned>>,
    #[uniform(unbound)]
    transform: Uniform<M44>,
}

#[derive(Copy, Clone, Debug, Semantics)]
pub enum Semantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexColor")]
    Color,
    #[sem(name = "tex_coord", repr = "[f32; 2]", wrapper = "VertexTexCoord")]
    TexCoord,
}

#[repr(C)]
#[derive(Vertex)]
#[vertex(sem = "Semantics")]
struct Vertex {
    pos: VertexPosition,
    #[vertex(normalized = "true")]
    color: VertexColor,
    #[vertex(normalized = "true")]
    tex_coord: VertexTexCoord,
}

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

enum Tag {
    Player,
}

struct GameObject {
    quad: Tess,
    tag: Tag,
    pos: Vector3<f32>,
    vel: Vector3<f32>,
    acc: Vector3<f32>,
}

impl GameObject {
    pub fn new(
        surface: &mut GlutinSurface,
        width: u32,
        height: u32,
        tag: Tag,
        pos: (f32, f32),
    ) -> Self {
        let (width, window_w) = (width as f32, WIDTH as f32);
        let (height, window_h) = (height as f32, HEIGHT as f32);

        let (top_left, top_right, bot_left, bot_right) = {
            let top_left = [-(width / window_w) / 2.0, -(height / window_h) / 2.0];
            let top_right = [(width / window_w) / 2.0, -(height / window_h) / 2.0];
            let bot_right = [(width / window_w) / 2.0, (height / window_h) / 2.0];
            let bot_left = [-(width / window_w) / 2.0, (height / window_h) / 2.0];

            (top_left, top_right, bot_left, bot_right)
        };
        // println!("TL:{:?}, TR{:?}, BR{:?}, BL{:?}", top_left, top_right, bot_right, bot_left);
        let vertices = [
            Vertex {
                pos: VertexPosition::new(top_left),
                color: VertexColor::new([255, 0, 0]),
                tex_coord: VertexTexCoord::new([0.0, 0.0]),
            },
            Vertex {
                pos: VertexPosition::new(top_right),
                color: VertexColor::new([0, 255, 0]),
                tex_coord: VertexTexCoord::new([1.0, 0.0]),
            },
            Vertex {
                pos: VertexPosition::new(bot_right),
                color: VertexColor::new([0, 0, 255]),
                tex_coord: VertexTexCoord::new([1.0, 1.0]),
            },
            Vertex {
                pos: VertexPosition::new(bot_left),
                color: VertexColor::new([255, 0, 255]),
                tex_coord: VertexTexCoord::new([0.0, 1.0]),
            },
        ];

        let quad = TessBuilder::new(surface)
            .add_vertices(vertices)
            .set_mode(Mode::TriangleFan)
            .build()
            .unwrap();

        let pos = Vector3::new(pos.0, pos.1, 0.0);
        let vel = Vector3::new(0.0, 0.0, 0.0);
        let acc = Vector3::new(0.0, 0.0, 0.0);

        GameObject {
            quad,
            tag,
            pos,
            vel,
            acc,
        }
    }

    pub fn update_acc(&mut self, acc: Vector3<f32>) {
        if acc[0] != 0.0 || acc[1] != 0.0 {
            self.acc = acc.normalize();
        } else {
            self.acc = Vector3::new(0.0, 0.0, 0.0);
        }
    }

    pub fn update_vel(&mut self, delta: f32) {
        self.vel += self.acc * delta;
        if self.vel.magnitude() > 0.2 {
            self.vel = self.vel.normalize() * 0.2;
        }
    }

    pub fn update_pos(&mut self, delta: f32) {
        self.pos += self.vel * delta;
    }
}

#[derive(Debug)]
struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

impl PlayerInput {
    pub fn new() -> Self {
        PlayerInput {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

fn main() {
    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(WIDTH, HEIGHT),
        "Asteroids",
        WindowOpt::default(),
    )
    .expect("Could not create GlutinSurface.");

    let (tex, tex_width, tex_height) =
        load_texture(&mut surface, Path::new("assets/images/ship_01.png"));

    let program = Program::<Semantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("Could not create Program.")
        .ignore_warnings();

    let render_st = RenderState::default();

    let back_buffer = surface.back_buffer().unwrap();

    let mut player = GameObject::new(&mut surface, tex_width, tex_height, Tag::Player, (0.0, 0.0));
    let mut input_manager = PlayerInput::new();

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
        let mut updated_acc = Vector3::new(0.0, 0.0, 0.0);
        if input_manager.up {
            updated_acc[1] += 1.0
        }
        if input_manager.down {
            updated_acc[1] -= 1.0
        }
        if input_manager.left {
            updated_acc[0] -= 1.0
        }
        if input_manager.right {
            updated_acc[0] += 1.0
        }
        player.update_acc(updated_acc);
        player.update_vel(frametime);
        player.update_pos(frametime);

        // Rendering
        surface.pipeline_builder().pipeline(
            &back_buffer,
            background_color,
            |pipeline, mut shd_gate| {
                let bound_tex = pipeline.bind_texture(&tex);

                shd_gate.shade(&program, |iface, mut rdr_gate| {
                    let transform = Matrix4::from_translation(player.pos);

                    iface.tex.update(&bound_tex);
                    iface.transform.update(transform.into());

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

fn load_texture(
    surface: &mut GlutinSurface,
    path: &Path,
) -> (Texture<Flat, Dim2, NormRGBA8UI>, u32, u32) {
    let img = image::open(path)
        .map(|img| img.flipv().to_rgba())
        .expect("Could not create image.");

    let (width, height) = img.dimensions();
    let texels = img.into_raw();

    let tex = Texture::new(surface, [width, height], 0, Sampler::default())
        .expect("Failed to create Texture.");

    tex.upload_raw(GenMipmaps::No, &texels).unwrap();

    (tex, width, height)
}
