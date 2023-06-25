use crate::{
    shape,
    shapes::aabb::AxisAlignedBoundingBox,
    utils::{
        hittable::{self, Hittable},
        ray::Ray,
        vec::{Point3, Vec3},
    },
};

shape!(Triangle {
    a: Point3,
    b: Point3,
    c: Point3,
    normal: Vec3
});

impl Hittable for Triangle {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<hittable::HitRecord> {
        let v0v1 = self.b - self.a;
        let v0v2 = self.c - self.a;
        let normal = v0v1.cross(&v0v2);

        let n_dot_ray_direction = normal.dot(&ray.direction);
        if n_dot_ray_direction.abs() < 1e-8 {
            return None;
        }

        let d = -normal.dot(&self.a);

        let t = -(normal.dot(&ray.origin) + d) / n_dot_ray_direction;

        if t < t_max && t > t_min {
            let point = ray.at(t);
            let mut edge = self.b - self.a;
            let mut vp = point - self.a;
            let mut c = edge.cross(&vp);
            if normal.dot(&c) < 0.0 {
                return None;
            }

            edge = self.c - self.b;
            vp = point - self.b;
            c = edge.cross(&vp);
            if normal.dot(&c) < 0.0 {
                return None;
            }

            edge = self.a - self.c;
            vp = point - self.c;
            c = edge.cross(&vp);
            if normal.dot(&c) < 0.0 {
                return None;
            }
            let (u, v) = self.get_triangle_uv(&point);
            let mut hit_record =
                hittable::HitRecord::new(point, normal, self.material.clone(), t, u, v);
            hit_record.set_face_normal(ray, normal);
            return Some(hit_record);
        }
        None
    }
    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AxisAlignedBoundingBox> {
        let small = Point3::new(
            self.a.x.min(self.b.x.min(self.c.x)),
            self.a.y.min(self.b.y.min(self.c.y)),
            self.a.z.min(self.b.z.min(self.c.z)),
        );
        let big = Point3::new(
            self.a.x.max(self.b.x.max(self.c.x)),
            self.a.y.max(self.b.y.max(self.c.y)),
            self.a.z.max(self.b.z.max(self.c.z)),
        );
        Some(AxisAlignedBoundingBox::new(small, big, None))
    }
}

impl Triangle {
    fn get_triangle_uv(&self, point: &Point3) -> (f32, f32) {
        let edge0 = self.b - self.a;
        let edge1 = self.c - self.a;
        let p = *point - self.a;
        let mut u = edge0.dot(&edge0);
        let mut v = edge0.dot(&edge1);
        let w = edge0.dot(&p);
        let uu = edge1.dot(&edge1);
        let uv = edge1.dot(&p);
        let denom = u * uu - v * v;
        u = (uu * w - v * uv) / denom;
        v = (u * v - uu * uv) / denom;
        (u, v)
    }
}
