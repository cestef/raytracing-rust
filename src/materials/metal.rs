use crate::{
    material,
    utils::{helpers::random_in_unit_sphere, hittable::HitRecord, ray::Ray, vec::Color},
};

use super::Material;

material!(Metal {
    albedo: Color,
    fuzz: f32
});

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = r_in.direction.unit_vector().reflect(&rec.normal);
        *scattered = Ray::new(
            rec.point,
            reflected + self.fuzz * random_in_unit_sphere().unit_vector(),
            r_in.time,
        );
        *attenuation = self.albedo;

        scattered.direction.dot(&rec.normal) > 0.0
    }
}
