use std::env;

use crate::{
    shapes::list::HittableList,
    utils::{
        camera::Camera,
        result::Res,
        vec::{Color, Vec3},
    },
};

pub fn random_float() -> f32 {
    rand::random::<f32>()
}

pub fn random_float_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random_float()
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random_range(-1.0, 1.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_vector() -> Vec3 {
    random_in_unit_sphere().unit_vector()
}

pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if in_unit_sphere.dot(normal) > 0.0 {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Use Schlick's approximation for reflectance.
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0.powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
// Return the chunks and each chunk size
pub fn split_evenly<T>(vec: Vec<T>, n: usize) -> (Vec<Vec<T>>, Vec<usize>)
where
    T: Clone,
{
    let mut chunks = Vec::with_capacity(n);
    let len = vec.len();
    let chunk_size = (len as f64 / n as f64).ceil() as usize;
    let mut chunk = Vec::with_capacity(chunk_size);
    let mut chunk_sizes = Vec::with_capacity(n);
    for (i, item) in vec.into_iter().enumerate() {
        if i != 0 && i % chunk_size == 0 {
            chunks.push(chunk);
            chunk = Vec::with_capacity(chunk_size);
        }
        chunk.push(item);
    }
    chunks.push(chunk);
    for chunk in &chunks {
        chunk_sizes.push(chunk.len());
    }
    (chunks, chunk_sizes)
}
pub fn parse_aspect_ratio(aspect_ratio: &str) -> Result<f32, String> {
    let splitters = vec![":", "/", "x"];
    for splitter in splitters {
        if aspect_ratio.contains(splitter) {
            let mut split = aspect_ratio.split(splitter);
            let width = split
                .next()
                .expect("Failed to get width from aspect_ratio")
                .parse::<i32>()
                .expect("Failed to parse width from aspect_ratio");
            let height = split
                .next()
                .expect("Failed to get height from aspect_ratio")
                .parse::<i32>()
                .expect("Failed to parse height from aspect_ratio");
            let ratio = width as f32 / height as f32;
            return Ok(ratio);
        }
    }
    Err("Failed to parse aspect ratio".to_string())
}

pub fn compute_chunk(args: (Vec<i32>, Box<Camera>, Box<HittableList>, i32, i32, i32)) -> Res {
    let (chunk, camera, world, image_width, image_height, samples_per_pixel) = args;
    let mut buffers = Vec::with_capacity(chunk.len());
    let start_index = chunk[0];
    for j in chunk {
        let mut buffer = Vec::with_capacity(image_width as usize);
        for i in 0..image_width {
            let mut average_color = Color::default();

            for _ in 0..samples_per_pixel {
                let u = (i as f32 + random_float()) / (image_width - 1) as f32; // 0.0 <= u <= 1.0 | u is the horizontal component of the pixel
                let v = (j as f32 + random_float()) / (image_height - 1) as f32; // 0.0 <= v <= 1.0 | v is the vertical component of the pixel
                let r = camera.get_ray(u, v);
                average_color += r.color(&world, 1000);
            }

            average_color /= samples_per_pixel as f32;
            average_color = average_color.clamp(0.0, 0.999);
            average_color = average_color.gamma_correct(2.0);
            buffer.push(average_color);
        }
        buffers.push(buffer);
    }

    Res {
        buffers,
        start: start_index,
    }
}

pub fn clear() {
    if env::consts::OS == "macos" || env::consts::OS == "linux" {
        std::process::Command::new("clear").status().unwrap();
    } else {
        std::process::Command::new("cls").status().unwrap();
    }
}
