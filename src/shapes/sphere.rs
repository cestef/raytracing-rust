use crate::{
    shape,
    utils::{
        hittable::{HitRecord, Hittable},
        ray::Ray,
        vec::Point3,
    },
};

shape!(Sphere {
    center: Point3,
    radius: f32
});

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }
        let discriminant = discriminant.sqrt();
        let t = (-half_b - discriminant) / a;
        if t < t_max && t > t_min {
            let point = ray.at(t);
            let normal = (point - self.center) / self.radius;
            let mut hit_record = HitRecord::new(point, normal, self.material.clone(), t);
            hit_record.set_face_normal(ray, normal);
            return Some(hit_record);
        }
        None
    }
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}
