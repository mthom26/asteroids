use ultraviolet::{mat::Mat4, projection::rh_yup::orthographic_gl, vec::Vec3};

use crate::utils::convert_mat4;

pub struct Camera {
    pub pos: Vec3,
    pub look_at: Vec3,
    pub up: Vec3,
    pub aspect_ratio: f32,
    pub clip_near: f32,
    pub clip_far: f32,
}

impl Camera {
    pub fn new(
        pos: Vec3,
        look_at: Vec3,
        up: Vec3,
        aspect_ratio: f32,
        clip_near: f32,
        clip_far: f32,
    ) -> Self {
        Camera {
            pos,
            look_at,
            up,
            aspect_ratio,
            clip_near,
            clip_far,
        }
    }

    pub fn get_matrices(&self) -> ([[f32; 4]; 4], [[f32; 4]; 4]) {
        let view = Mat4::look_at(self.pos, self.look_at, self.up);
        let view_matrix = convert_mat4(view);

        let projection = orthographic_gl(
            -self.aspect_ratio,
            self.aspect_ratio,
            -1.0,
            1.0,
            self.clip_near,
            self.clip_far,
        );
        let projection_matrix = convert_mat4(projection);

        (view_matrix, projection_matrix)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: Vec3::new(0.0, 0.0, 1.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            up: Vec3::new(0.0, 1.0, 0.0),
            aspect_ratio: 16.0 / 9.0,
            clip_near: 0.1,
            clip_far: 100.0,
        }
    }
}
