use std::{
    thread,
    path::Path,
    time::{Duration, Instant},
};

use image;

use luminance::{
    context::GraphicsContext,
    pipeline::BoundTexture,
    pixel::{NormRGBA8UI, NormUnsigned},
    render_state::RenderState,
    shader::program::{Program, Uniform},
    tess::{Mode, TessBuilder},
    texture::{Dim2, Flat, GenMipmaps, Sampler, Texture},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glutin::{
    ElementState, Event, GlutinSurface, KeyboardInput, Surface, VirtualKeyCode, WindowDim,
    WindowEvent, WindowOpt,
};

const VS: &'static str = include_str!("../assets/shaders/vertex.glsl");
const FS: &'static str = include_str!("../assets/shaders/fragment.glsl");

#[derive(UniformInterface)]
struct ShaderInterface {
    // the 'static lifetime acts as “anything” here
    tex: Uniform<&'static BoundTexture<'static, Flat, Dim2, NormUnsigned>>,
}

#[derive(Copy, Clone, Debug, Semantics)]
pub enum Semantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexColor")]
    Color,
}

#[derive(Vertex)]
#[vertex(sem = "Semantics")]
struct Vertex {
    pos: VertexPosition,
    #[vertex(normalized = "true")]
    color: VertexColor,
}

const WIDTH: u32 = 1280;
const HEIGHT: u32 = 720;

fn main() {
    let mut surface = GlutinSurface::new(
        WindowDim::Windowed(WIDTH, HEIGHT),
        "Asteroids",
        WindowOpt::default(),
    )
    .expect("Could not create GlutinSurface.");

    let (tex, tex_width, tex_height) = load_texture(&mut surface, Path::new("assets/images/ship_01.png"));


    let program = Program::<Semantics, (), ShaderInterface>::from_strings(None, VS, None, FS)
        .expect("Could not create Program.")
        .ignore_warnings();

    let render_st = RenderState::default();

    let (top_left, top_right, bot_left, bot_right) = {
        let top_left = [-(tex_width as f32 / WIDTH as f32) / 2.0, -(tex_height as f32 / HEIGHT as f32) / 2.0];
        let top_right = [(tex_width as f32 / WIDTH as f32) / 2.0, -(tex_height as f32 / HEIGHT as f32) / 2.0];
        let bot_right = [(tex_width as f32 / WIDTH as f32) / 2.0, (tex_height as f32 / HEIGHT as f32) / 2.0];
        let bot_left = [-(tex_width as f32 / WIDTH as f32) / 2.0, (tex_height as f32 / HEIGHT as f32) / 2.0];

        (top_left, top_right, bot_left, bot_right)
    };

    let vertices = [
        Vertex {
            pos: VertexPosition::new(top_left),
            color: VertexColor::new([255, 0, 0]),
        },
        Vertex {
            pos: VertexPosition::new(top_right),
            color: VertexColor::new([0, 255, 0]),
        },
        Vertex {
            pos: VertexPosition::new(bot_right),
            color: VertexColor::new([0, 0, 255]),
        },
        Vertex {
            pos: VertexPosition::new(bot_left),
            color: VertexColor::new([255, 0, 255]),
        },
    ];

    println!("TL:{:?}, TR{:?}, BR{:?}, BL{:?}", top_left, top_right, bot_right, bot_left);

    let quad = TessBuilder::new(&mut surface)
        .add_vertices(vertices)
        .set_mode(Mode::TriangleFan)
        .build()
        .unwrap();
    
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
            .pipeline(&back_buffer, background_color, |pipeline, mut shd_gate| {
                let bound_tex = pipeline.bind_texture(&tex);

                shd_gate.shade(&program, |iface, mut rdr_gate| {
                    iface.tex.update(&bound_tex);

                    rdr_gate.render(render_st, |mut tess_gate| {
                        tess_gate.render(&quad);
                    });
                });
            });

        surface.swap_buffers();

        // Limit Framerate
        thread::sleep(Duration::new(0, 1000000000 / 60));
        total_frames += 1;
    }

    println!("Total frames rendered: {}", total_frames);
}

fn load_texture(surface: &mut GlutinSurface, path: &Path) -> (Texture<Flat, Dim2, NormRGBA8UI>, u32, u32) {
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
