use crate::Vec3;
use crate::Ray;
use crate::hit;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl hit::Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut hit::Hit) -> bool {
        let origin_to_center = ray.origin() - &self.center;
        let a = ray.direction().squared_length();
        let half_b = origin_to_center.dot(ray.direction());
        let c = origin_to_center.squared_length() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant > 0. {
            let root = discriminant.sqrt();
            let temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.temp = temp;
                rec.p = ray.point_at_parameter(temp);
                rec.normal = (&rec.p - &self.center) / self.radius;
                return true;
            }
            let temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.temp = temp;
                rec.p = ray.point_at_parameter(temp);
                rec.normal = (&rec.p - &self.center) / self.radius;
                return true;
            }
        }
        false
    }
}
