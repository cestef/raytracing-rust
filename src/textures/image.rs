use image::{io::Reader, GenericImageView};

use crate::{
    textures::Texture,
    utils::vec::{Color, Point3, Vec3},
};

#[derive(Clone, Debug)]
pub struct ImageTexture {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _point: &Point3) -> Vec3 {
        if self.data.is_empty() {
            return Color::new(1.0, 0.0, 0.0);
        }
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0);
        let mut i = (u * self.width as f32) as usize;
        let mut j = (v * self.height as f32) as usize;

        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel = j * self.width + i;
        let r = self.data[pixel * 3] as f32 * color_scale;
        let g = self.data[pixel * 3 + 1] as f32 * color_scale;
        let b = self.data[pixel * 3 + 2] as f32 * color_scale;
        Color::new(r, g, b)
    }
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        let img = Reader::open(filename).unwrap().decode().unwrap();
        let (width, height) = img.dimensions();
        let data = img.into_rgb8().into_raw();
        Self {
            data,
            width: width as usize,
            height: height as usize,
        }
    }
}
