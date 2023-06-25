use crate::{
    material,
    materials::Material,
    utils::{
        helpers::{random_float, reflectance},
        hittable::HitRecord,
        ray::Ray,
        vec::Color,
    },
};

material!(Dielectric { n: f32 });

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face { 1.0 / self.n } else { self.n };
        let unit_direction = r_in.direction.unit_vector();

        let cos_theta = (-unit_direction).dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || reflectance(cos_theta, refraction_ratio) > random_float() {
                unit_direction.reflect(&rec.normal)
            } else {
                unit_direction.refract(&rec.normal, refraction_ratio)
            };

        *scattered = Ray::new(rec.point, direction, r_in.time);

        true
    }
}
