use std::rc::Rc;

use crate::Vec3;
use crate::Ray;

#[derive(Debug, Clone, Default)]
pub struct Hit {
    pub temp: f32,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Hit) -> bool;
}


#[derive(Default, Clone)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Hit) -> bool {
        let mut temp_rec = Hit::default();
        let mut closest_so_far = t_max;
        let mut hit_anything = false;

        for object in self.objects.iter() {
            let was_hit = object.hit(ray, t_min, closest_so_far, &mut temp_rec);
            if was_hit {
                hit_anything = true;
                closest_so_far = temp_rec.temp;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}