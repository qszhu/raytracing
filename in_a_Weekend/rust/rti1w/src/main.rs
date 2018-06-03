extern crate rand;
mod vec3;
mod ray;
mod hitable;
mod camera;
mod material;

use std::f32;
use rand::Rng;

use vec3::{Vec3, Color};
use ray::Ray;
use hitable::{Hitable, HitableList, Sphere};
use camera::Camera;
use material::{Lambertian, Metal};

fn color(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, f32::MAX) {
        if depth < 50 {
            if let Some(scatter) = hit.mat_ptr.scatter(r, &hit) {
                return scatter.attenuation * color(&scatter.scattered, world, depth+1);
            }
        }
        return Vec3::new(0.,0.,0.);
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
    let material1 = Lambertian::new(Vec3::new(0.8, 0.3, 0.3));
    let sphere1 = Sphere::new(Vec3::new(0.,0.,-1.), 0.5, Box::new(material1));
    list.push(Box::new(sphere1));
    let material2 = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let sphere2 = Sphere::new(Vec3::new(0.,-100.5,-1.), 100., Box::new(material2));
    list.push(Box::new(sphere2));
    let material3 = Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0);
    let sphere3 = Sphere::new(Vec3::new(1.,0.,-1.), 0.5, Box::new(material3));
    list.push(Box::new(sphere3));
    let material4 = Metal::new(Vec3::new(0.8, 0.8, 0.8), 0.3);
    let sphere4 = Sphere::new(Vec3::new(-1.,0.,-1.), 0.5, Box::new(material4));
    list.push(Box::new(sphere4));
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
                col += color(&r, &world, 0);
            }
            col /= ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let c = Color::from_vec3(&col);
            println!("{} {} {}", c.r, c.g, c.b);
        }
    }
}
