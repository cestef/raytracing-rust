use std::fmt::{Debug, Formatter, Result};

use dyn_clonable::clonable;

use crate::{
    materials::Material,
    shapes::aabb::AxisAlignedBoundingBox,
    utils::{
        ray::Ray,
        vec::{Point3, Vec3},
    },
};

#[clonable]
pub trait Hittable: Clone + Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AxisAlignedBoundingBox>;
}

impl Debug for dyn Hittable {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("Hittable").finish()
    }
}

#[derive(Debug)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Option<Box<dyn Material + Send + Sync>>,
    pub u: f32, // used for texture mapping
    pub v: f32, // used for texture mapping
}

impl HitRecord {
    pub fn new(
        point: Point3,
        normal: Vec3,
        material: Option<Box<dyn Material + Send + Sync>>,
        t: f32,
        u: f32,
        v: f32,
    ) -> Self {
        Self {
            point,
            normal,
            material,
            t,
            front_face: false,
            u,
            v,
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
