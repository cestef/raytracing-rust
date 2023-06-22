use crate::{
    material,
    materials::Material,
    utils::{helpers::random_vector, hittable::HitRecord, ray::Ray, vec::Color},
};

material!(Lambertian { albedo: Color });

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let scatter_direction = rec.normal + random_vector();

        *scattered = Ray::new(rec.point, scatter_direction);
        *attenuation = self.albedo;

        true
    }
}
