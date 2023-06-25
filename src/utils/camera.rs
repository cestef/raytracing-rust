use crate::utils::{
    helpers::random_float_range,
    ray::Ray,
    vec::{Point3, Vec3},
};

pub const MIN_TIME: f32 = 0.0;
pub const MAX_TIME: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            origin: Point3::default(),
            lower_left_corner: Point3::default(),
            horizontal: Vec3::default(),
            vertical: Vec3::default(),
        }
    }
}

impl Camera {
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        v_up: Vec3,
        v_fov: f32,
        aspect_ratio: f32,
    ) -> Self {
        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(&w).unit_vector();
        let v = w.cross(&u);
        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }
    // Get a ray from the camera to the pixel at (u, v)
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
            random_float_range(MIN_TIME, MAX_TIME),
        )
    }
}
