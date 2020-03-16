use ray_tracing::Vec3;

fn main() {
    let nx = 200;
    let ny = 200;
    println!("P3\n{} {}\n255", nx, ny);

    for j in (0 .. ny).rev() {
        for i in 0 .. nx {
            let r = (i as f32) / (nx as f32);
            let g = (j as f32) / (ny as f32);
            let b = 0.2;

            let col = Vec3::new(r, g, b);

            let ir = (255.99 * col.r()) as isize;
            let ig = (255.99 * col.g()) as isize;
            let ib = (255.99 * col.b()) as isize;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
