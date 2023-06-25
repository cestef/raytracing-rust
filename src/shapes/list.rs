use crate::{
    shapes::aabb::AxisAlignedBoundingBox,
    utils::{
        hittable::{HitRecord, Hittable},
        ray::Ray,
    },
};

#[derive(Default, Clone, Debug)]
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
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
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
    fn bounding_box(&self, time0: f32, time1: f32) -> Option<AxisAlignedBoundingBox> {
        if self.objects.is_empty() {
            return None;
        }
        let mut output_box = AxisAlignedBoundingBox::default();
        let mut first_box = true;
        for object in &self.objects {
            if let Some(temp_box) = object.bounding_box(time0, time1) {
                output_box = if first_box {
                    temp_box
                } else {
                    AxisAlignedBoundingBox::surrounding_box(output_box, temp_box)
                };
                first_box = false;
            } else {
                return None;
            }
        }
        Some(output_box)
    }
}
