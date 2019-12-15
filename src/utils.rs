use ultraviolet::mat::Mat4;

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
