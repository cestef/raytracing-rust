use crate::{
    texture,
    utils::vec::{Point3, Vec3},
};

use super::Texture;

texture!(CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>
});

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, point: &Point3) -> Vec3 {
        let sines = f32::sin(10.0 * point.x) * f32::sin(10.0 * point.y) * f32::sin(10.0 * point.z);
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}
