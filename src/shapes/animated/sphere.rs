use crate::{
    shape,
    shapes::{aabb::AxisAlignedBoundingBox, sphere::Sphere},
    utils::{
        hittable::{HitRecord, Hittable},
        ray::Ray,
        vec::Point3,
    },
};

// A sphere that moves from center0 to center1 between time0 and time1.
shape!(MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f32,
    time1: f32,
    radius: f32
});

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center(ray.time);
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
            let normal = (point - self.center(ray.time)) / self.radius;
            let (u, v) = Sphere::get_sphere_uv(normal);
            let mut hit_record = HitRecord::new(point, normal, self.material.clone(), t, u, v);
            hit_record.set_face_normal(ray, normal);
            return Some(hit_record);
        }
        None
    }
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AxisAlignedBoundingBox> {
        let box0 = AxisAlignedBoundingBox::new(
            self.center(time0) - Point3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Point3::new(self.radius, self.radius, self.radius),
            None,
        );
        let box1 = AxisAlignedBoundingBox::new(
            self.center(time1) - Point3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Point3::new(self.radius, self.radius, self.radius),
            None,
        );
        Some(AxisAlignedBoundingBox::surrounding_box(box0, box1))
    }
}

impl MovingSphere {
    pub fn center(&self, time: f32) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}
