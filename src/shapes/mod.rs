pub mod aabb;
pub mod animated;
pub mod bvh;
pub mod list;
pub mod plane;
pub mod sphere;
pub mod triangle;

// Declare shape struct with material
/*
shape!(Sphere {
    center: Point3,
    radius: f32,
})
 */
#[macro_export]
macro_rules! shape {
    ($name:ident {
        $($field:ident: $type:ty),*
    }) => {
        #[derive(Clone)]
        pub struct $name {
            pub material: Option<Box<dyn crate::materials::Material + Send + Sync>>,
            $(pub $field: $type),*
        }

        impl $name {
            pub fn new($($field: $type),*, material: Option<Box<dyn crate::materials::Material + Send + Sync>>) -> Self {
                Self {
                    material,
                    $($field),*
                }
            }
        }
    };
}
