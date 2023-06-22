use std::fmt::{Display, Error, Formatter};

use crate::{
    shapes::list::HittableList,
    utils::{
        hittable::Hittable,
        vec::{Point3, Vec3},
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point3 {
        self.origin + self.direction * t
    }
    pub fn color(&self, world: &HittableList, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::zero();
        }
        match world.hit(self, 1e-8, std::f32::INFINITY) {
            Some(hit) => {
                let mut scattered = Ray::default();
                let mut attenuation = Vec3::default();
                if hit
                    .material
                    .scatter(self, &hit, &mut attenuation, &mut scattered)
                {
                    attenuation * scattered.color(world, depth - 1)
                } else {
                    Vec3::zero()
                }
            }
            None => {
                let unit_direction = self.direction.unit_vector();
                let t = 0.5 * (unit_direction.y + 1.0);
                (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
            }
        }
    }
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Point3::default(),
            direction: Vec3::default(),
        }
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{{ origin: {}, direction: {} }}",
            self.origin, self.direction
        )
    }
}
