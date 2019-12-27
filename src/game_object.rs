use crate::Tag;

use luminance::tess::Tess;
use luminance_glutin::GlutinSurface;

use ultraviolet::{rotor::Rotor3, vec::Vec3};

use crate::utils::quad;

pub struct GameObject {
    pub quad: Tess,
    pub tag: Tag,
    pub pos: Vec3,
    pub vel: Vec3,
    pub acc: Vec3,
    pub rot: f32,
}

impl GameObject {
    pub fn new(surface: &mut GlutinSurface, tag: Tag, pos: (f32, f32), scale: f32) -> Self {
        let quad = quad(surface, None, scale);

        let pos = Vec3::new(pos.0, pos.1, 0.0);
        let vel = Vec3::new(0.0, 0.0, 0.0);
        let acc = Vec3::new(0.0, 0.0, 0.0);

        let rot = 0.0;

        GameObject {
            quad,
            tag,
            pos,
            vel,
            acc,
            rot,
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

    // Rotate towards a target
    fn update_rot(&mut self, target_pos: Vec3, delta: f32) {
        let target = (target_pos - self.pos).normalized();
        let rot = Rotor3::from_rotation_xy(self.rot);
        let mut current = Vec3::new(0.0, -1.0, 0.0);
        current.rotate_by(rot);

        let target_rot = Rotor3::from_rotation_between(current, target);
        let target_rot = 2.0 * target_rot.bv.xy.asin();
        // println!("{}", target_rot);

        let rate = 3.0; // Rotation rate
                        // TODO - Smooth rate when rotation approaches the target
        let rot = match target_rot {
            val if val > 0.0 => rate,
            val if val < 0.0 => -rate,
            _ => 0.0,
        };

        self.rot += rot * delta;
    }

    // Convenience function to handle all updates
    pub fn update(&mut self, delta: f32, acc: Vec3, rotation_target: Vec3) {
        self.update_rot(rotation_target, delta);
        self.update_acc(acc);
        self.update_vel(delta);
        self.update_pos(delta);
    }
}
