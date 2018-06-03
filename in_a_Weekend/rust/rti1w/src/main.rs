extern crate rand;
mod vec3;
mod ray;
mod hitable;
mod camera;

use std::f32;
use rand::Rng;

use vec3::{Vec3, Color};
use ray::Ray;
use hitable::{Hitable, HitableList, Sphere};
use camera::Camera;

fn color(r: &Ray, world: &Hitable) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.0, f32::MAX) {
        return 0.5*Vec3::new(hit.normal.x+1., hit.normal.y+1., hit.normal.z+1.);
    } else {
        let unit_direction = Vec3::unit(r.direction);
        let t = 0.5*(unit_direction.y + 1.0);
        return (1.0-t)*Vec3::new(1., 1., 1.) + t*Vec3::new(0.5, 0.7, 1.);
    }
}

fn main() {
    let (nx, ny, ns) = (200, 100, 100);
    println!("P3\n{} {}\n255", nx, ny);

    let mut list: Vec<Box<Hitable>> = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.,0.,-1.), 0.5)));
    list.push(Box::new(Sphere::new(Vec3::new(0.,-100.5,-1.), 100.)));
    let world = HitableList::new(list);

    let cam = Camera::new();

    let mut rng = rand::thread_rng();
    let mut r = || -> f32 { rng.gen() };
    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _s in 0..ns {
                let u = (i as f32 + r()) / nx as f32;
                let v = (j as f32 + r()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &world);
            }
            col /= ns as f32;
            let c = Color::from_vec3(&col);
            println!("{} {} {}", c.r, c.g, c.b);
        }
    }
}
