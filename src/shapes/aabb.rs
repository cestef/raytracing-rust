use crate::{
    shape,
    utils::{
        hittable::{HitRecord, Hittable},
        ray,
        vec::Point3,
    },
};

shape!(AxisAlignedBoundingBox {
    min: Point3,
    max: Point3
});

impl Hittable for AxisAlignedBoundingBox {
    fn hit(&self, ray: &ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut t_min = t_min;
        let mut t_max = t_max;

        for a in 0..3 {
            let inv_d = 1.0 / ray.direction[a];
            let mut t0 = (self.min[a] - ray.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - ray.origin[a]) * inv_d;

            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }

            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t1 < t_max { t1 } else { t_max };

            if t_max <= t_min {
                return None;
            }
        }

        let point = ray.at(t_min);
        let outward_normal = (point - self.min).unit_vector();
        let (u, v) = self.get_aabb_uv(&point);
        let mut hit_record = HitRecord::new(point, outward_normal, None, t_min, u, v);
        hit_record.set_face_normal(ray, outward_normal);
        Some(hit_record)
    }
    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AxisAlignedBoundingBox> {
        Some(self.clone())
    }
}

impl AxisAlignedBoundingBox {
    pub fn surrounding_box(box0: AxisAlignedBoundingBox, box1: AxisAlignedBoundingBox) -> Self {
        let small = Point3::new(
            box0.min.x.min(box1.min.x),
            box0.min.y.min(box1.min.y),
            box0.min.z.min(box1.min.z),
        );
        let big = Point3::new(
            box0.max.x.max(box1.max.x),
            box0.max.y.max(box1.max.y),
            box0.max.z.max(box1.max.z),
        );
        Self::new(small, big, None)
    }
    fn get_aabb_uv(&self, point: &Point3) -> (f32, f32) {
        let u = (point.x - self.min.x) / (self.max.x - self.min.x);
        let v = (point.y - self.min.y) / (self.max.y - self.min.y);
        (u, v)
    }
}

impl Default for AxisAlignedBoundingBox {
    fn default() -> Self {
        Self::new(
            Point3::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY),
            Point3::new(
                std::f32::NEG_INFINITY,
                std::f32::NEG_INFINITY,
                std::f32::NEG_INFINITY,
            ),
            None,
        )
    }
}
