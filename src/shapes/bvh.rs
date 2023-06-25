use std::cmp::Ordering;

use crate::{
    shape,
    shapes::aabb::AxisAlignedBoundingBox,
    utils::{
        helpers::{box_x_compare, box_y_compare, box_z_compare},
        hittable::{HitRecord, Hittable},
        ray::Ray,
    },
};

shape!(BvhNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>,
    bounding_box: AxisAlignedBoundingBox
});

impl Hittable for BvhNode {
    fn bounding_box(&self, _time0: f32, _time1: f32) -> Option<AxisAlignedBoundingBox> {
        Some(self.bounding_box.clone())
    }
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bounding_box.hit(ray, t_min, t_max).is_none() {
            return None;
        }
        let hit_left = self.left.hit(ray, t_min, t_max);
        let hit_right = self.right.hit(ray, t_min, t_max);
        match (hit_left, hit_right) {
            (Some(hit_left), Some(hit_right)) => {
                if hit_left.t < hit_right.t {
                    Some(hit_left)
                } else {
                    Some(hit_right)
                }
            }
            (Some(hit_left), None) => Some(hit_left),
            (None, Some(hit_right)) => Some(hit_right),
            (None, None) => None,
        }
    }
}

impl BvhNode {
    /// Create a new BvhNode from a list of objects
    /// # Arguments
    /// * `objects` - A list of objects to create the BvhNode from
    /// * `start` - The start index of the objects
    /// * `end` - The end index of the objects
    /// * `time0` - The start time
    /// * `time1` - The end time
    pub fn from_objects(
        objects: Vec<Box<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f32,
        time1: f32,
    ) -> Self {
        let axis = rand::random::<u8>() % 3;
        let mut objects = objects;
        let comparator = match axis {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };
        let object_span = objects.len();

        let (left, right) = match object_span {
            1 => {
                let left = objects.remove(0);
                (left.clone(), left.clone()) as (Box<dyn Hittable>, Box<dyn Hittable>)
            }
            2 => {
                if comparator(&objects[0], &objects[1]) == Ordering::Less {
                    (objects.remove(0), objects.remove(0)) as (Box<dyn Hittable>, Box<dyn Hittable>)
                } else {
                    (objects.remove(1), objects.remove(0)) as (Box<dyn Hittable>, Box<dyn Hittable>)
                }
            }
            _ => {
                objects.sort_by(comparator);
                let mid = object_span / 2;
                let left_objects = objects.drain(..mid).collect::<Vec<_>>();
                let right_objects = objects.drain(..).collect::<Vec<_>>();
                (
                    Box::new(BvhNode::from_objects(
                        left_objects,
                        start,
                        end,
                        time0,
                        time1,
                    )) as Box<dyn Hittable>,
                    Box::new(BvhNode::from_objects(
                        right_objects,
                        start,
                        end,
                        time0,
                        time1,
                    )) as Box<dyn Hittable>,
                )
            }
        };
        let left_box = left.bounding_box(time0, time1).unwrap();
        let right_box = right.bounding_box(time0, time1).unwrap();
        let bounding_box = AxisAlignedBoundingBox::surrounding_box(left_box, right_box);
        BvhNode {
            left,
            right,
            bounding_box,
            material: None,
        }
    }
}
