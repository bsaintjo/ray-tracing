use crate::Vec3;
use crate::Ray;

struct Hit {
    t: f32,
    p: Vec3,
    normal: Vec3,
}

trait Hittable {
    fn hit(self, ray: &Ray, t_min: f32, t_max: f32, rec: &Hit) -> bool;
}
