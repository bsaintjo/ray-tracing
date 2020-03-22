use ray_tracing::{Ray, Vec3};

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> Option<f32> {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(&oc) - radius *radius;
    let discriminant = (b * b) - (4.0 * a * c);
    if discriminant < 0.0 {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2.0 * a))
    }
}

fn color(ray: &Ray) -> Vec3 {
    let center = Vec3::new(0., 0., -1.);
    let radius = 0.5;
    if let Some(t) = hit_sphere(center, radius, ray) {
        let pos = Vec3::new(0., 0., -1.);
        let big_n = (ray.point_at_parameter(t) - pos).unit_vector();
        0.5 * (big_n + 1.)
    } else {
        let unit = ray.direction().clone().unit_vector();
        let t = 0.5 * (unit.y() + 1.0);
        let one_vec3 = Vec3::new(1.0, 1.0, 1.0);
        let other = Vec3::new(0.5, 0.7, 1.0);
        (1.0 - t) * one_vec3 + (other * t)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0 .. ny).rev() {
        for i in 0 .. nx {
            let u = (i as f32) / (nx as f32);
            let v = (j as f32) / (ny as f32);
            let r = Ray::new(origin.clone(), &lower_left_corner + u * &horizontal + v * &vertical);
            let col = color(&r);

            let ir = (255.99 * col.index(0)) as isize;
            let ig = (255.99 * col.index(1)) as isize;
            let ib = (255.99 * col.index(2)) as isize;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
