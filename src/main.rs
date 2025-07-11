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
use std::time::Instant;
use std::{fs::File, rc::Rc};
use utils::{random_double, random_double_range};
use vec3::Vec3;

fn main() -> std::io::Result<()> {
    let current = Instant::now();
    println!("Started rendering image");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let samples_per_pixel = 400;
    let max_depth = 20;
    let vfov = 20.0;
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        v_up,
        defocus_angle,
        focus_dist,
    );

    let mat_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        mat_ground,
    )));

    for i in -11..11 {
        for j in -11..11 {
            let mat_choice = random_double();
            let center = Vec3::new(
                f64::from(i) + 0.9 * random_double(),
                0.2,
                f64::from(j) + 0.9 * random_double(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if mat_choice < 0.8 {
                    let albedo = Color::random() * (Color::random());
                    let sphere_mat = Rc::new(Lambertian::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
                } else if mat_choice < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let sphere_mat = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
                } else {
                    let sphere_mat = Rc::new(Dielectric::new(1.50));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_mat)));
                }
            }
        }
    }

    let mat1 = Rc::new(Dielectric::new(1.50));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, mat1)));

    let mat2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, mat2)));

    let mat3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, mat3)));

    let mut file = File::create("output.ppm")?;
    camera.render(&world, &mut file)?;
    let elapsed = current.elapsed();
    println!("Elapsed time : {elapsed:.4?}");

    Ok(())
}
