extern crate rand;

use camera::Camera;
use hitable::Hitable;
use hitable::Sphere;
use material::*;
use ray::Ray;
use std::f32;
use std::time::Instant;
use vec3::Vec3;
use std::rc::Rc;

mod vec3;
mod ray;
mod material;
mod camera;
mod hitable;
mod util;

const GRAY: Vec3 = Vec3 { x: 0.5, y: 0.5, z: 0.5 };
const RED: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
const WIKTOR_RED: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
const GREEN: Vec3 = Vec3 { x: 0.1, y: 0.9, z: 0.1 };
const BLUE: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };
const WHITE: Vec3 = Vec3 { x: 1.0, y: 1.0, z: 1.0 };

fn main() {
    let now = Instant::now();


    let args: Vec<String> = std::env::args().collect();

    let width = args.get(1).unwrap().parse().unwrap();
    let height = args.get(2).unwrap().parse().unwrap();
    let num_samples: u32 = args.get(3).unwrap_or(&"1".to_string()).parse().unwrap();

    let vfov_deg = 50.0;

    println!("P3");
    println!("{} {}", width, height);
    println!("255");

    let look_from = Vec3::new(3.0, 1.2, 1.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        vfov_deg,
        width as f32 / height as f32,
        0.5,
        (look_from - look_at).length(),
    );

    let spheres = book_scene();
    let spheres = random_scene();
    let spheres = my_scene();

    for j in 0..height {
        for i in 0..width {
            let j = height - 1 - j;

            let mut c = Vec3::ZERO;
            for _ in 0..num_samples {
                let u = (i as f32 + util::rand_f32()) / width as f32;
                let v = (j as f32 + util::rand_f32()) / height as f32;

                let r = camera.get_ray(u, v);
                c += color(&r, &spheres, 50);
            }
            c = c / num_samples as f32;
            let gamma = 2.0;
            c.x = c.x.powf(1.0 / gamma);
            c.y = c.y.powf(1.0 / gamma);
            c.z = c.z.powf(1.0 / gamma);
            let ir = (c.r() * 255.999) as i32;
            let ig = (c.g() * 255.999) as i32;
            let ib = (c.b() * 255.999) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }

    let elapsed = now.elapsed();
    let secs = elapsed.as_secs();
    let subsec_millis = elapsed.subsec_nanos() / 1000000;
    eprintln!("Elapsed {}.{}s", secs, subsec_millis);
}

fn color<H: Hitable>(r: &Ray, hitable: &H, hits_left: u8) -> Vec3 {
    if let Some(hit) = hitable.hit(r, 0.001, f32::MAX) {
        if hits_left > 0 {
            let material = hit.material.clone();
            if let Some((reflected_ray, absorption)) = material.scatter(r, &hit) {
                let mut c = color(&reflected_ray, hitable, hits_left - 1);
                c.x *= absorption.x;
                c.y *= absorption.y;
                c.z *= absorption.z;
                return c;
            } else {
                return Vec3::ZERO;
            }
        }
    }
    let unit_dir = r.direction.normalize();
    let alpha = unit_dir.y * 0.5 + 1.0;
    let white = Vec3::ONES;
    let light_blue = Vec3::new(0.5, 0.7, 1.0);
    white * (1.0 - alpha) + (light_blue * alpha)
}

fn my_scene() -> Vec<Sphere> {
    vec![
        Sphere { center: Vec3::new(0.0, 0.0, -1.0), r: 0.5, material: Rc::new(Metal { albedo: GRAY, fuzz: 0.01 }) },
        Sphere { center: Vec3::new(0.0, -100.5, -1.0), r: 100.0, material: Rc::new(Lambertian { albedo: GRAY }) },
        Sphere { center: Vec3::new(-1.5, 0.0, -1.0), r: 0.5, material: Rc::new(Lambertian { albedo: RED }) },
        Sphere { center: Vec3::new(0.0, -0.4, -0.5), r: 0.1, material: Rc::new(Lambertian { albedo: GREEN }) },
        Sphere { center: Vec3::new(0.5, -0.4, -0.6), r: 0.1, material: Rc::new(Metal { albedo: WIKTOR_RED, fuzz: 0.005 }) },
        Sphere { center: Vec3::new(-0.24, -0.4, -0.55), r: 0.1, material: Rc::new(Glass { refraction_index: 1.5 }) },
        Sphere { center: Vec3::new(-0.24, -0.4, -0.55), r: -0.07, material: Rc::new(Glass { refraction_index: 1.5 }) },
        Sphere { center: Vec3::new(-0.24, -0.4, 0.25), r: 0.13, material: Rc::new(Metal { albedo: WHITE, fuzz: 1.0 }) },
    ]
}

fn book_scene() -> Vec<Sphere> {
    let mat1 = Rc::new(Lambertian { albedo: Vec3::new(0.1, 0.2, 0.5) });
    let mat2 = Rc::new(Lambertian { albedo: Vec3::new(0.8, 0.8, 0.0) });
    let mat3 = Rc::new(Metal { albedo: Vec3::new(0.8, 0.6, 0.2), fuzz: 0.0 });
    let mat4 = Rc::new(Glass { refraction_index: 1.5 });
    let mat5 = mat4.clone();
    vec![
        Sphere { center: Vec3::new(0.0, 0.0, -1.0), r: 0.5, material: mat1 },
        Sphere { center: Vec3::new(0.0, -100.5, -1.0), r: 100.0, material: mat2 },
        Sphere { center: Vec3::new(1.0, 0.0, -1.0), r: 0.5, material: mat3 },
        Sphere { center: Vec3::new(-1.0, 0.0, -1.0), r: 0.5, material: mat4 },
        Sphere { center: Vec3::new(-1.0, 0.0, -1.0), r: -0.45, material: mat5 },
    ]
}

fn random_scene() -> Vec<Sphere> {
    let mat1 = Rc::new(Glass { refraction_index: 1.5 });
    let mat2 = Rc::new(Lambertian { albedo: Vec3::new(0.8, 0.2, 0.2) });
    let mat3 = Rc::new(Metal { albedo: Vec3::new(0.1, 0.1, 0.9), fuzz: 0.05 });
    let mat_floor = Rc::new(Lambertian { albedo: Vec3::new(0.4, 0.4, 0.4) });

    let sphere_base = Sphere { center: Vec3::new(0.0, -1000.0, 0.0), r: 1000.0, material: mat_floor };
    let sphere1 = Sphere { center: Vec3::new(1.0, 0.5, -2.32), r: 0.5, material: mat1 };
    let sphere2 = Sphere { center: Vec3::new(0.0, 0.5, -1.4), r: 0.5, material: mat2 };
    let sphere3 = Sphere { center: Vec3::new(-0.4, 0.5, 0.5), r: 0.5, material: mat3 };


    let mut spheres = Vec::new();

    spheres.push(sphere_base);
    spheres.push(sphere1);
    spheres.push(sphere2);
    spheres.push(sphere3);

    for a in -2..3 {
        for b in -2..3 {
            let choose_mat = util::rand_f32();
            let c_x = a as f32 + util::rand_f32() * 0.9;
            let c_y = 0.1 + util::rand_f32() * 0.3;
            let c_z = b as f32 + util::rand_f32() * 0.9;
            let center = Vec3::new(c_x, c_y, c_z);
            let material: Rc<Material>;
            if choose_mat < 0.4 {
                let albedo = Vec3::new(
                    util::rand_f32().powi(2),
                    util::rand_f32().powi(2),
                    util::rand_f32().powi(2),
                );
                material = Rc::new(Lambertian { albedo });
            } else if choose_mat > 0.95 {
                let refraction_index = 1.0 + util::rand_f32();
                material = Rc::new(Glass { refraction_index });
            } else {
                let albedo = Vec3::new(
                    0.5 * (util::rand_f32() + 1.0),
                    0.5 * (util::rand_f32() + 1.0),
                    0.5 * (util::rand_f32() + 1.0),
                );
                let fuzz = util::rand_f32().powi(2);
                material = Rc::new(Metal { albedo, fuzz });
            }
            let sphere = Sphere {
                center,
                r: c_y,
                material,
            };
            spheres.push(sphere);
        }
    }
    spheres
}
