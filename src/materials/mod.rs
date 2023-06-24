use std::fmt::Debug;

use dyn_clonable::clonable;

use crate::utils::{hittable::HitRecord, ray::Ray, vec::Color};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

#[clonable]
pub trait Material: Clone + Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

impl Debug for dyn Material {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Material").finish()
    }
}

#[macro_export]
macro_rules! material {
    ($name:ident {
        $($field:ident: $type:ty),*
    }) => {
        #[derive(Clone, Copy)]
        pub struct $name {
            $(pub $field: $type),*
        }

        impl $name {
            pub fn new($($field: $type),*) -> Self {
                Self {
                    $($field),*
                }
            }
        }
    };
}
