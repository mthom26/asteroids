use std::path::Path;

use luminance::{
    pixel::NormRGBA8UI,
    tess::{Mode, Tess, TessBuilder},
    texture::{Dim2, Flat, GenMipmaps, Sampler, Texture},
};
use luminance_glutin::GlutinSurface;

use ultraviolet::mat::Mat4;

use crate::rendering::Vertex;

// Utility function to convert the ultraviolet `Mat4` to the needed `M44` for the
// shader.
pub fn convert_mat4(mat: Mat4) -> [[f32; 4]; 4] {
    [
        [mat.cols[0].x, mat.cols[0].y, mat.cols[0].z, mat.cols[0].w],
        [mat.cols[1].x, mat.cols[1].y, mat.cols[1].z, mat.cols[1].w],
        [mat.cols[2].x, mat.cols[2].y, mat.cols[2].z, mat.cols[2].w],
        [mat.cols[3].x, mat.cols[3].y, mat.cols[3].z, mat.cols[3].w],
    ]
}

// Return a new quad with initial uniform scaling.
pub fn quad(surface: &mut GlutinSurface, vertices: Option<&[Vertex]>, scale: f32) -> Tess {
    let default = [
        Vertex::from([-scale, -scale], [0.0, 0.0]), // Bottom left
        Vertex::from([scale, -scale], [1.0, 0.0]),  // Bottom right
        Vertex::from([scale, scale], [1.0, 1.0]),   // Top right
        Vertex::from([-scale, scale], [0.0, 1.0]),  // Top left
    ];

    let verts = match vertices {
        Some(vertices) => vertices,
        // No vertices supplied so return a default
        None => &default,
    };

    TessBuilder::new(surface)
        .add_vertices(verts)
        .set_mode(Mode::TriangleFan)
        .build()
        .unwrap()
}

pub fn load_texture(
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
