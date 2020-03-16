use ray_tracing::{Ray, Vec3};

fn hit_sphere(center: &Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(ray.direction());
    let b = 2.0 * oc.dot(ray.direction());
    let c = oc.dot(&oc) - radius.powf(radius);
    let discriminant = b.powf(b) - 4.0 * a * c;
    discriminant > 0.0
}

fn color(ray: &Ray) -> Vec3 {
    todo!()
}

fn main() {}
