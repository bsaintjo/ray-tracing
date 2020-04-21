use ray_tracing::Vec3;

fn main() {
    let image_width = 200;
    let image_height = 200;
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0 .. image_height).rev() {
        eprint!("\rScanlines remaining: {}", j);
        for i in 0 .. image_width {
            let r = (i as f32) / (image_width as f32);
            let g = (j as f32) / (image_height as f32);
            let b = 0.2;

            let color = Vec3::new(r, g, b);
            color.write_color();
        }
    }
    eprintln!("\nDone.");
}
