mod vec3;

use vec3::{Vec3, Color};

fn main() {
    let (nx, ny) = (200, 100);
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let col = Color::from_vec3(&Vec3::new(i as f32 / nx as f32, j as f32 / ny as f32, 0.2));
            println!("{} {} {}", col.r, col.g, col.b);
        }
    }
}
