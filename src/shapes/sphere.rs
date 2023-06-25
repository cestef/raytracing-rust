use crate::{
    shape,
    utils::{
        hittable::{HitRecord, Hittable},
        ray::Ray,
        vec::Point3,
    },
};

use super::aabb::AxisAlignedBoundingBox;

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
            let (u, v) = Self::get_sphere_uv(normal);
            let mut hit_record = HitRecord::new(point, normal, self.material.clone(), t, u, v);
            hit_record.set_face_normal(ray, normal);
            return Some(hit_record);
        }
        None
    }
    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AxisAlignedBoundingBox> {
        Some(AxisAlignedBoundingBox::new(
            self.center - Point3::new(self.radius, self.radius, self.radius),
            self.center + Point3::new(self.radius, self.radius, self.radius),
            None,
        ))
    }
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}

impl Sphere {
    /// Gets the uv coordinates of a sphere
    /// # Arguments
    /// * `p` - The point to get the uv coordinates of
    /// # Returns
    /// A tuple containing the uv coordinates
    /// * u: returned value [0,1] of angle around the Y axis from X=-1.
    /// * v: returned value [0,1] of angle from Y=-1 to Y=+1.
    /// # Examples
    /// *    `<1 0 0>` yields `<0.50 0.50>`       `<-1  0  0>` yields `<0.00 0.50>`
    /// *    `<0 1 0>` yields `<0.50 1.00>`       `< 0 -1  0>` yields `<0.50 0.00>`
    /// *    `<0 0 1>` yields `<0.25 0.50>`       `< 0  0 -1>` yields `<0.75 0.50>`
    pub fn get_sphere_uv(p: Point3) -> (f32, f32) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + std::f32::consts::PI;
        (
            phi / (2.0 * std::f32::consts::PI),
            theta / std::f32::consts::PI,
        )
    }
}
