use impl_ops::*;
use std::ops;
use std::ops::{Neg};

#[derive(Clone, Debug)]
pub struct Vec3 {
    es: [f32; 3]
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        let es = [e0, e1, e2];
        Vec3 { es }
    }

    pub fn x(&self) -> f32 {
        self.es[0]
    }

    pub fn y(&self) -> f32 {
        self.es[1]
    }

    pub fn z(&self) -> f32 {
        self.es[2]
    }

    pub fn r(&self) -> f32 {
        self.es[0]
    }

    pub fn g(&self) -> f32 {
        self.es[1]
    }

    pub fn b(&self) -> f32 {
        self.es[2]
    }

    pub fn index(&self, idx: usize) -> f32 {
        self.es[idx]
    }

    pub fn length(&self) -> f32 {
        (self.es[0].powi(2) + self.es[1].powi(2) + self.es[2].powi(2)).sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.es[0].powi(2) + self.es[1].powi(2) + self.es[2].powi(2)
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        (self.x() * other.x()) + (self.y() * other.y()) + (self.z() * other.z())
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        let e0 = (self.y() * other.z()) - (self.z() * other.y());
        let e1 = (self.z() * other.x()) - (self.x() * other.z());
        let e2 = (self.x() * other.y()) - (self.y() * other.x());
        Vec3::new(e0, e1, e2)
    }

    // Normalizes a Vec3 by its length
    pub fn unit_vector(self) -> Vec3 {
        let len = self.length();
        self / len
    }
}

impl_ops::impl_op_ex_commutative!(+ |a: &Vec3, b: &f32| -> Vec3 {
        let e0 = a.x() + b;
        let e1 = a.y() + b;
        let e2 = a.z() + b;
        Vec3::new(e0, e1, e2)
});

impl_ops::impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 {
        let e0 = a.x() + b.x();
        let e1 = a.y() + b.y();
        let e2 = a.z() + b.z();
        Vec3::new(e0, e1, e2)
});

impl_ops::impl_op_ex!(* |a: &Vec3, b: &Vec3| -> Vec3 {
        let e0 = a.x() * b.x();
        let e1 = a.y() * b.y();
        let e2 = a.z() * b.z();
        Vec3::new(e0, e1, e2)
});

impl_ops::impl_op_ex_commutative!(* |a: &Vec3, b: &f32| -> Vec3 {
        let e0 = a.x() * b;
        let e1 = a.y() * b;
        let e2 = a.z() * b;
        Vec3::new(e0, e1, e2)
});

impl_ops::impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| {
    let e0 = a.x() + b.x();
    let e1 = a.y() + b.y();
    let e2 = a.z() + b.z();
    *a = Vec3::new(e0, e1, e2);
});

impl_ops::impl_op_ex!(*= |a: &mut Vec3, b: &Vec3| {
    let e0 = a.x() * b.x();
    let e1 = a.y() * b.y();
    let e2 = a.z() * b.z();
    *a = Vec3::new(e0, e1, e2);
});

impl_ops::impl_op_ex!(*= |a: &mut Vec3, b: &f32| {
    let e0 = a.x() * b;
    let e1 = a.y() * b;
    let e2 = a.z() * b;
    *a = Vec3::new(e0, e1, e2);
});

impl_ops::impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 {
    let e0 = a.x() / b.x();
    let e1 = a.y() / b.y();
    let e2 = a.z() / b.z();
    Vec3::new(e0, e1, e2)
});

impl_ops::impl_op_ex!(/ |a: &Vec3, b: &f32| -> Vec3 {
    let e0 = a.x() / b; let e1 = a.y() / b;
    let e2 = a.z() / b;
    Vec3::new(e0, e1, e2)
});

impl_ops::impl_op_ex!(/= |a: &mut Vec3, b: &Vec3| {
    let e0 = a.x() / b.x();
    let e1 = a.y() / b.y();
    let e2 = a.z() / b.z();
    *a = Vec3::new(e0, e1, e2)
});

impl_ops::impl_op_ex!(/= |a: &mut Vec3, b: &f32| {
    let e0 = a.x() / b;
    let e1 = a.y() / b;
    let e2 = a.z() / b;
    *a = Vec3::new(e0, e1, e2)
});

impl_ops::impl_op_ex!(- |a: &Vec3, b: &f32| -> Vec3 {
    let e0 = a.x() - b;
    let e1 = a.y() - b;
    let e2 = a.z() - b;
    Vec3::new(e0, e1, e2)
});

impl_ops::impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 {
    let e0 = a.x() - b.x();
    let e1 = a.y() - b.y();
    let e2 = a.z() - b.z();
    Vec3::new(e0, e1, e2)
});

impl_ops::impl_op_ex!(-= |a: &mut Vec3, b: &f32| {
    let e0 = a.x() - b;
    let e1 = a.y() - b;
    let e2 = a.z() - b;
    *a = Vec3::new(e0, e1, e2);
});

impl_ops::impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| {
    let e0 = a.x() - b.x();
    let e1 = a.y() - b.y();
    let e2 = a.z() - b.z();
    *a = Vec3::new(e0, e1, e2);
});


impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        let e0 = -self.x();
        let e1 = -self.y();
        let e2 = -self.z();
        Vec3::new(e0, e1, e2)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
