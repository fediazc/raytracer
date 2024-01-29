use std::rc::Rc;

use crate::{
    color::Color,
    interval::Interval,
    material::{Lambertian, Material},
    ray::Ray,
    vec3::{dot, Point3, Vec3},
};

pub struct HitRecord {
    pub hit: bool,
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Option<Rc<dyn Material>>,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            hit: false,
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: None,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> HitRecord;
}
