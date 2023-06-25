use crate::{
    textures::Texture,
    utils::vec::{Point3, Vec3},
};
use noise::{NoiseFn, Perlin};

#[derive(Clone)]
pub struct NoiseTexture {
    pub scale: f32,
    pub octaves: usize,
    pub frequency: f32,
    pub persistence: f32,
    pub lacunarity: f32,
    perlin: Perlin,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, point: &Point3) -> Vec3 {
        Vec3::new(1.0, 1.0, 1.0) * self.scale * self.noise(point)
    }
}

impl NoiseTexture {
    pub fn new(
        scale: f32,
        octaves: usize,
        frequency: f32,
        persistence: f32,
        lacunarity: f32,
    ) -> Self {
        Self {
            scale,
            perlin: Perlin::new(rand::random::<u32>()),
            octaves,
            frequency,
            persistence,
            lacunarity,
        }
    }
    pub fn with_scale(mut self, scale: f32) -> Self {
        self.scale = scale;
        self
    }
    pub fn with_octaves(mut self, octaves: usize) -> Self {
        self.octaves = octaves;
        self
    }
    pub fn with_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }
    pub fn with_persistence(mut self, persistence: f32) -> Self {
        self.persistence = persistence;
        self
    }
    pub fn with_lacunarity(mut self, lacunarity: f32) -> Self {
        self.lacunarity = lacunarity;
        self
    }
    fn noise(&self, point: &Point3) -> f32 {
        // Based on tutorial from https://flafla2.github.io/2014/08/09/perlinnoise.html
        // Given an octave i we define:
        //   frequency = 2^i
        //   amplitude = persistence * i
        let mut x = point.x;
        let mut y = point.y;
        let mut z = point.z;
        let mut amplitude = self.persistence;
        let mut frequency = self.frequency;

        let mut total = 0.0;
        let mut max_value = 0.0; // Used to normalize results to [0.0-1.0]

        for _ in 0..self.octaves {
            let perlin_value = self.perlin.get([
                (x * self.frequency) as f64,
                (y * self.frequency) as f64,
                (z * self.frequency) as f64,
            ]) as f32;

            total += perlin_value * amplitude;
            max_value += amplitude;

            amplitude *= self.persistence;
            frequency *= frequency;

            x *= self.lacunarity;
            y *= self.lacunarity;
            z *= self.lacunarity;
        }

        (total / max_value) as f32
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new(1.0, 4, 1.0, 0.5, 2.0)
    }
}
