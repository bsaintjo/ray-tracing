use crate::Vec3;
use crate::Ray;

pub struct Hit {
    pub temp: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Hit) -> bool;
}
