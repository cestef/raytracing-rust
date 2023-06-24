use crate::{
    shape,
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
/*
bool rayTriangleIntersect(
    const Vec3f &orig, const Vec3f &dir,
    const Vec3f &v0, const Vec3f &v1, const Vec3f &v2,
    float &t)
{
    // compute the plane's normal
    Vec3f v0v1 = v1 - v0;
    Vec3f v0v2 = v2 - v0;
    // no need to normalize
    Vec3f N = v0v1.crossProduct(v0v2); // N
    float area2 = N.length();

    // Step 1: finding P

    // check if the ray and plane are parallel.
    float NdotRayDirection = N.dotProduct(dir);
    if (fabs(NdotRayDirection) < kEpsilon) // almost 0
        return false; // they are parallel, so they don't intersect!

    // compute d parameter using equation 2
    float d = -N.dotProduct(v0);

    // compute t (equation 3)
    t = -(N.dotProduct(orig) + d) / NdotRayDirection;

    // check if the triangle is behind the ray
    if (t < 0) return false; // the triangle is behind

    // compute the intersection point using equation 1
    Vec3f P = orig + t * dir;

    // Step 2: inside-outside test
    Vec3f C; // vector perpendicular to triangle's plane

    // edge 0
    Vec3f edge0 = v1 - v0;
    Vec3f vp0 = P - v0;
    C = edge0.crossProduct(vp0);
    if (N.dotProduct(C) < 0) return false; // P is on the right side

    // edge 1
    Vec3f edge1 = v2 - v1;
    Vec3f vp1 = P - v1;
    C = edge1.crossProduct(vp1);
    if (N.dotProduct(C) < 0)  return false; // P is on the right side

    // edge 2
    Vec3f edge2 = v0 - v2;
    Vec3f vp2 = P - v2;
    C = edge2.crossProduct(vp2);
    if (N.dotProduct(C) < 0) return false; // P is on the right side;

    return true; // this ray hits the triangle
}
 */
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

            let mut hit_record = hittable::HitRecord::new(point, normal, self.material.clone(), t);
            hit_record.set_face_normal(ray, normal);
            return Some(hit_record);
        }
        None
    }
}
