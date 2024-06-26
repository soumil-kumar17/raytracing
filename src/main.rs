mod camera;
mod color;
mod hittable;
mod interval;
mod material;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::HittableList;
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use std::{f64::consts::PI, fs::File, rc::Rc};
use vec3::Vec3;

fn main() -> std::io::Result<()> {
    use std::time::Instant;

    let current = Instant::now();
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 90.0;
    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
    );

    //let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    //let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let mat_left = Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0)));
    //let mat_bubble = Rc::new(Dielectric::new(1.00 / 1.50));
    let mat_right = Rc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0)));

    let r = f64::cos(PI / 4.0);

    let mut world = HittableList::new();
    // world.add(Rc::new(Sphere::new(
    //     Vec3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     mat_ground,
    // )));
    // world.add(Rc::new(Sphere::new(
    //     Vec3::new(0.0, 0.0, -1.2),
    //     0.5,
    //     mat_center,
    // )));
    world.add(Rc::new(Sphere::new(Vec3::new(-r, 0.0, -1.0), r, mat_left)));
    // world.add(Rc::new(Sphere::new(
    //     Vec3::new(-1.0, 0.0, -1.0),
    //     0.4,
    //     mat_bubble,
    // )));
    world.add(Rc::new(Sphere::new(Vec3::new(r, 0.0, -1.0), r, mat_right)));

    let mut file = File::create("output.ppm")?;
    camera.render(&world, &mut file)?;
    let elapsed = current.elapsed();
    println!("Elapsed time : {:.4?}", elapsed);
    Ok(())
}
