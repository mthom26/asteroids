use crate::Tag;

use luminance::tess::{Mode, Tess, TessBuilder};
use luminance_glutin::GlutinSurface;

use ultraviolet::vec::Vec3;

use crate::rendering::Vertex;

pub struct GameObject {
    pub quad: Tess,
    pub tag: Tag,
    pub pos: Vec3,
    pub vel: Vec3,
    pub acc: Vec3,
}

impl GameObject {
    pub fn new(
        surface: &mut GlutinSurface,
        width: u32,
        height: u32,
        tag: Tag,
        pos: (f32, f32),
    ) -> Self {
        let (width, window_w) = (width as f32, crate::WIDTH as f32);
        let (height, window_h) = (height as f32, crate::HEIGHT as f32);

        let (top_left, top_right, bot_left, bot_right) = {
            let top_left = [-(width / window_w) / 2.0, -(height / window_h) / 2.0];
            let top_right = [(width / window_w) / 2.0, -(height / window_h) / 2.0];
            let bot_right = [(width / window_w) / 2.0, (height / window_h) / 2.0];
            let bot_left = [-(width / window_w) / 2.0, (height / window_h) / 2.0];

            (top_left, top_right, bot_left, bot_right)
        };
        // println!("TL:{:?}, TR{:?}, BR{:?}, BL{:?}", top_left, top_right, bot_right, bot_left);
        let vertices = [
            Vertex::from(top_left, [255, 0, 0], [0.0, 0.0]),
            Vertex::from(top_right, [0, 255, 0], [1.0, 0.0]),
            Vertex::from(bot_right, [0, 0, 255], [1.0, 1.0]),
            Vertex::from(bot_left, [255, 0, 255], [0.0, 1.0]),
        ];

        let quad = TessBuilder::new(surface)
            .add_vertices(vertices)
            .set_mode(Mode::TriangleFan)
            .build()
            .unwrap();

        let pos = Vec3::new(pos.0, pos.1, 0.0);
        let vel = Vec3::new(0.0, 0.0, 0.0);
        let acc = Vec3::new(0.0, 0.0, 0.0);

        GameObject {
            quad,
            tag,
            pos,
            vel,
            acc,
        }
    }

    fn update_acc(&mut self, acc: Vec3) {
        if acc[0] != 0.0 || acc[1] != 0.0 {
            self.acc = acc.normalized();
        } else {
            self.acc = Vec3::new(0.0, 0.0, 0.0);
        }
    }

    fn update_vel(&mut self, delta: f32) {
        self.vel += self.acc * delta;
        if self.vel.mag() > 0.2 {
            self.vel = self.vel.normalized() * 0.2;
        }
    }

    fn update_pos(&mut self, delta: f32) {
        self.pos += self.vel * delta;
    }

    // Convenience function to handle all updates
    pub fn update(&mut self, delta: f32, acc: Vec3) {
        self.update_acc(acc);
        self.update_vel(delta);
        self.update_pos(delta);
    }
}
