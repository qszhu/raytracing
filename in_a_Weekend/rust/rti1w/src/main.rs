mod vec3;
mod ray;

use vec3::{Vec3, Color};
use ray::Ray;

fn color(r: Ray) -> Vec3 {
    let unit_direction = Vec3::unit(r.direction);
    let t = 0.5*(unit_direction.y + 1.0);
    return (1.0-t)*Vec3::new(1., 1., 1.) + t*Vec3::new(0.5, 0.7, 1.);
}

fn main() {
    let (nx, ny) = (200, 100);
    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);
            let col = Color::from_vec3(&color(r));
            println!("{} {} {}", col.r, col.g, col.b);
        }
    }
}
