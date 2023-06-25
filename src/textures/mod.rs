use dyn_clonable::clonable;

use crate::utils::vec::{Point3, Vec3};

pub mod checker;
pub mod image;
pub mod noise;
pub mod solid;

#[clonable]
pub trait Texture: Send + Sync + Clone {
    fn value(&self, u: f32, v: f32, point: &Point3) -> Vec3;
}

#[macro_export]
macro_rules! texture {
    ($name:ident {
        $($field:ident: $type:ty),*
    }) => {
        #[derive(Clone)]
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
