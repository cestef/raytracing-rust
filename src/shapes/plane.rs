use crate::{
    shape,
    shapes::aabb::AxisAlignedBoundingBox,
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
            let (u, v) = self.get_plane_uv(&point);
            let mut hit_record = HitRecord::new(point, self.normal, self.material.clone(), t, u, v);
            hit_record.set_face_normal(ray, self.normal);
            return Some(hit_record);
        }
        None
    }
    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AxisAlignedBoundingBox> {
        None
    }
}

impl Plane {
    fn get_plane_uv(&self, point: &Point3) -> (f32, f32) {
        let u = (point.x - self.center.x) / 2.0;
        let v = (point.z - self.center.z) / 2.0;
        (u, v)
    }
}
