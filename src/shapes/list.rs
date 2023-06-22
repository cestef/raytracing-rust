use crate::utils::hittable::{HitRecord, Hittable};

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Box<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &crate::utils::ray::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(temp_hit_record) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = temp_hit_record.t;
                hit_record = Some(temp_hit_record);
            }
        }
        hit_record
    }
}
