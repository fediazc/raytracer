mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use color::Color;
use hittable_list::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::Vec3;

fn main() {
    let mut world = HittableList::new();

    let mat_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let mat_center = Dielectric::new(1.5);
    let mat_left = Dielectric::new(1.5);
    let mat_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, mat_left));
    world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, mat_center));
    world.add(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, mat_right));
    world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, mat_ground));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.render(&world);
}
