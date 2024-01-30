use crate::{
    color::{write_color, Color},
    hittable::Hittable,
    interval::Interval,
    ray::Ray,
    utils::{degrees_to_radians, randf},
    vec3::{cross, random_in_unit_disk, unit_vector},
    vec3::{Point3, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,

    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    image_height: i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Vec3::new(0.0, 0.0, -1.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
            image_height: 100,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, self.max_depth, world);
                }

                write_color(&pixel_color, self.samples_per_pixel);
            }
        }
        eprintln!("\rDone.                    ");
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_center =
            self.pixel00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = if self.defocus_angle < 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();

        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + randf();
        let py = -0.5 + randf();

        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let true_aspect_ratio = self.image_width as f64 / self.image_height as f64;
        let viewport_width = viewport_height * true_aspect_ratio;

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * degrees_to_radians(self.defocus_angle / 2.0).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }
}

fn ray_color(r: &Ray, depth: i32, world: &impl Hittable) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let hit_rec = world.hit(r, &Interval::new(0.001, f64::INFINITY));

    if hit_rec.hit {
        let mut scattered_ray = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
        let mut attenuation = Color::new(0.0, 0.0, 0.0);

        let did_scatter = match &hit_rec.mat {
            Some(mat) => mat.scatter(r, &hit_rec, &mut attenuation, &mut scattered_ray),
            _ => panic!("Ray hit Hittable with no material"),
        };

        if did_scatter {
            return attenuation * ray_color(&scattered_ray, depth - 1, world);
        }

        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y + 1.0);

    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
