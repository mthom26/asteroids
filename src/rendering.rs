use luminance::{
    linear::M44,
    pipeline::BoundTexture,
    pixel::NormUnsigned,
    shader::program::Uniform,
    texture::{Dim2, Flat},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};

#[derive(UniformInterface)]
pub struct ShaderInterface {
    // the 'static lifetime acts as “anything” here
    pub tex: Uniform<&'static BoundTexture<'static, Flat, Dim2, NormUnsigned>>,
    pub model: Uniform<M44>,
    pub view: Uniform<M44>,
    pub proj: Uniform<M44>,
}

#[derive(Copy, Clone, Debug, Semantics)]
pub enum Semantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "tex_coord", repr = "[f32; 2]", wrapper = "VertexTexCoord")]
    TexCoord,
}

#[repr(C)]
#[derive(Vertex)]
#[vertex(sem = "Semantics")]
pub struct Vertex {
    pos: VertexPosition,
    #[vertex(normalized = "true")]
    tex_coord: VertexTexCoord,
}

impl Vertex {
    pub fn from(pos: [f32; 2], tex_coord: [f32; 2]) -> Self {
        Vertex {
            pos: VertexPosition::new(pos),
            tex_coord: VertexTexCoord::new(tex_coord),
        }
    }
}
