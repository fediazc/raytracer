use std::rc::Rc;

use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{dot, Point3},
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: impl Material + 'static) -> Self {
        Self {
            center,
            radius,
            mat: Rc::new(mat),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> HitRecord {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, &r.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        let mut hit_rec = HitRecord::new();

        hit_rec.hit = false;

        if discriminant < 0.0 {
            return hit_rec;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return hit_rec;
            }
        }

        hit_rec.hit = true;
        hit_rec.t = root;
        hit_rec.p = r.at(hit_rec.t);
        let outward_normal = (hit_rec.p - self.center) / self.radius;
        hit_rec.set_face_normal(&r, outward_normal);
        hit_rec.mat = Some(self.mat.clone());

        hit_rec
    }
}
