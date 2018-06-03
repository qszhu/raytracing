fn main() {
    let (nx, ny) = (200, 100);
    println!("P3\n{} {}\n255", nx, ny);
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2;
            let ir = (255.99*r) as u8;
            let ig = (255.99*g) as u8;
            let ib = (255.99*b) as u8;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
