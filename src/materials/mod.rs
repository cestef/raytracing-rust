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

impl Debug for dyn Material + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Material").finish()
    }
}

#[macro_export]
macro_rules! material {
    ($name:ident {
        $($field:ident: $type:ty),*
    }) => {
        #[derive(Clone)]
        pub struct $name {
            $(pub $field: $type),*,
            pub texture: Option<Box<dyn crate::textures::Texture + Send + Sync>>
        }

        impl $name {
            pub fn new($($field: $type),*) -> Self {
                Self {
                    $($field),*,
                    texture: None
                }
            }
            pub fn with_texture($($field: $type),*, texture: Box<dyn crate::textures::Texture + Send + Sync>) -> Self {
                Self {
                    $($field),*,
                    texture: Some(texture)
                }
            }
        }
    };
}
