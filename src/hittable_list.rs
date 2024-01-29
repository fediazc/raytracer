use std::vec::Vec;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: impl Hittable + 'static) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> HitRecord {
        let mut hit_rec = HitRecord::new();
        let mut closest_so_far = ray_t.max;

        hit_rec.hit = false;

        for object in &self.objects[..] {
            let temp_rec = object.hit(r, &Interval::new(ray_t.min, closest_so_far));
            if temp_rec.hit {
                closest_so_far = temp_rec.t;
                hit_rec = temp_rec;
            }
        }

        hit_rec
    }
}
