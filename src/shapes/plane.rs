use crate::{
    shape,
    utils::{
        hittable::{HitRecord, Hittable},
        ray::Ray,
        vec::{Point3, Vec3},
    },
};

shape!(Plane {
    center: Point3,
    normal: Vec3
});

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if ray.direction.dot(&self.normal).abs() < 1e-8 {
            return None;
        }
        let t = (self.center - ray.origin).dot(&self.normal) / ray.direction.dot(&self.normal);
        if t < t_max && t > t_min {
            let point = ray.at(t);
            let mut hit_record = HitRecord::new(point, self.normal, self.material.clone(), t);
            hit_record.set_face_normal(ray, self.normal);
            return Some(hit_record);
        }
        None
    }
}
