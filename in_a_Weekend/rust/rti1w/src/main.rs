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
use material::{Lambertian, Metal, Dielectric};

fn color(r: &Ray, world: &Hitable, depth: i32) -> Vec3 {
    if let Some(hit) = world.hit(r, 0.001, f32::MAX) {
        if depth > 0 {
            if let Some(scatter) = hit.mat_ptr.scatter(r, &hit) {
                return scatter.attenuation * color(&scatter.scattered, world, depth-1);
            }
        }
        return Vec3::new(0.,0.,0.);
    } else {
        let unit_direction = Vec3::unit(r.direction);
        let t = 0.5*(unit_direction.y + 1.0);
        return (1.0-t)*Vec3::new(1., 1., 1.) + t*Vec3::new(0.5, 0.7, 1.);
    }
}

fn random_scene() -> Box<Hitable> {
    let mut list: Vec<Box<Hitable>> = Vec::new();
    list.push(Box::new(
        Sphere::new(Vec3::new(0.,-1000.,0.), 1000., Box::new(
            Lambertian::new(Vec3::new(0.5, 0.5, 0.5))
        ))
    ));
    let mut rng = rand::thread_rng();
    let mut r = || -> f32 { rng.gen() };
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = r();
            let center = Vec3::new(a as f32+0.9*r(), 0.2, b as f32+0.9*r());
            if (center-Vec3::new(4.,0.2,0.)).length() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    list.push(Box::new(
                        Sphere::new(center, 0.2, Box::new(
                            Lambertian::new(Vec3::new(r()*r(), r()*r(), r()*r()))
                        ))
                    ));
                } else if choose_mat < 0.95 { // metal
                    list.push(Box::new(
                        Sphere::new(center, 0.2, Box::new(
                            Metal::new(Vec3::new(0.5*(1.+r()), 0.5*(1.+r()), 0.5*(1.+r())), 0.5*r())
                        ))
                    ));
                } else { // glass
                    list.push(Box::new(
                        Sphere::new(center, 0.2, Box::new(
                            Dielectric::new(1.5)
                        ))
                    ));
                }
            }
        }
    }
    list.push(Box::new(
        Sphere::new(Vec3::new(0.,1.,0.), 1., Box::new(
            Dielectric::new(1.5)
        ))
    ));
    list.push(Box::new(
        Sphere::new(Vec3::new(-4.,1.,0.), 1., Box::new(
            Lambertian::new(Vec3::new(0.4, 0.2, 0.1))
        ))
    ));
    list.push(Box::new(
        Sphere::new(Vec3::new(4.,1.,0.), 1., Box::new(
            Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.)
        ))
    ));
    Box::new(HitableList::new(list))
}

fn main() {
    let (nx, ny, ns, nd) = (200, 100, 100, 10);
    println!("P3\n{} {}\n255", nx, ny);

    let world = random_scene();

    let lookfrom = Vec3::new(13.,2.,3.);
    let lookat = Vec3::zero();
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let cam = Camera::new(lookfrom, lookat, Vec3::new(0.,1.,0.), 20., nx as f32 / ny as f32, aperture, dist_to_focus);

    let mut rng = rand::thread_rng();
    let mut r = || -> f32 { rng.gen() };
    for j in 0..ny {
        for i in 0..nx {
            let mut col = Vec3::new(0., 0., 0.);
            for _s in 0..ns {
                let u = (i as f32 + r()) / nx as f32;
                let v = ((ny-1-j) as f32 + r()) / ny as f32;
                let r = cam.get_ray(u, v);
                col += color(&r, &*world, nd);
            }
            col /= ns as f32;
            col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let c = Color::from_vec3(&col);
            println!("{} {} {}", c.r, c.g, c.b);
        }
    }
}
