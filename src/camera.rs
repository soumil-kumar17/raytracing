use std::{io::Write, ops::Neg};

use crate::{
    color::{write_color, Color},
    hittable::{HitRecord, Hittable},
    interval::Interval,
    ray::Ray,
    utils::{self, degrees_to_radians, random_double},
    vec3::Vec3,
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    samples_per_pixel: i32,
    pixel_samples_scale: f64,
    max_depth: i32,
    vfov: f64,
    look_from: Vec3,
    look_at: Vec3,
    v_up: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_angle: f64,
    focus_dist: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let mut camera = Camera {
            aspect_ratio,
            image_width,
            image_height: 0,
            center: Vec3::default(),
            pixel00_loc: Vec3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            samples_per_pixel,
            pixel_samples_scale: 0.0,
            max_depth,
            vfov,
            look_from,
            look_at,
            v_up,
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_angle,
            focus_dist,
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        };
        camera.initialize();

        camera
    }

    fn initialize(&mut self) {
        self.image_height = (f64::from(self.image_width) / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / f64::from(self.samples_per_pixel);
        self.center = self.look_from;

        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * self.aspect_ratio;

        self.w = Vec3::unit_vector(self.look_from - self.look_at);
        self.u = Vec3::unit_vector(self.v_up.cross(self.w));
        self.v = self.w.cross(self.u);

        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * self.v.neg();

        self.pixel_delta_u = viewport_u / f64::from(self.image_width);
        self.pixel_delta_v = viewport_v / f64::from(self.image_height);

        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        let defocus_radius =
            self.focus_dist * f64::tan(utils::degrees_to_radians(self.defocus_angle / 2.0));

        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    pub fn render(&self, world: &dyn Hittable, output: &mut dyn Write) -> std::io::Result<()> {
        writeln!(
            output,
            "P3\n{} {}\n255",
            self.image_width, self.image_height
        )?;

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += ray_color(&r, self.max_depth, world);
                }

                write_color(output, &(pixel_color * self.pixel_samples_scale))?;
            }
        }
        Ok(())
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = Camera::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((f64::from(i) + offset.x) * self.pixel_delta_u)
            + ((f64::from(j) + offset.y) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = Vec3::random_in_unit_disk();
        self.center + self.defocus_disk_u * p.x + self.defocus_disk_v * p.y
    }
}

fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::default();
    if world.hit(r, Interval::new(0.001, f64::INFINITY), &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        let mut rec_copy = rec.clone();
        if rec
            .mat
            .scatter(r, &mut rec_copy, &mut attenuation, &mut scattered)
        {
            return attenuation.elementwise_mul(ray_color(&scattered, depth - 1, world));
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_dir = Vec3::normalized(r.dir);
    let t = 0.5 * (unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
