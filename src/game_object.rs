use crate::Tag;

use luminance::tess::Tess;
use luminance_glutin::GlutinSurface;

use ultraviolet::vec::Vec3;

use crate::utils::quad;

pub struct GameObject {
    pub quad: Tess,
    pub tag: Tag,
    pub pos: Vec3,
    pub vel: Vec3,
    pub acc: Vec3,
}

impl GameObject {
    pub fn new(surface: &mut GlutinSurface, tag: Tag, pos: (f32, f32), scale: f32) -> Self {
        let quad = quad(surface, None, scale);

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
