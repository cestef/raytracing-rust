use crate::{
    texture,
    textures::Texture,
    utils::vec::{Color, Point3},
};

texture!(SolidColor { color: Color });

impl Texture for SolidColor {
    fn value(&self, _u: f32, _v: f32, _point: &Point3) -> Color {
        self.color
    }
}
