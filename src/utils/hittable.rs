use dyn_clonable::clonable;

use crate::{
    materials::Material,
    utils::{
        ray::Ray,
        vec::{Point3, Vec3},
    },
};

#[clonable]
pub trait Hittable: Clone + Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Box<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Point3,
        normal: Vec3,
        material: Box<dyn Material + Send + Sync>,
        t: f32,
    ) -> Self {
        Self {
            point,
            normal,
            material,
            t,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction.dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
