use super::float3::{Float3, Point3, Vector3};

#[derive(Debug, Copy, Clone, PartialEq)]

pub struct Ray {
    pub origin: Point3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + self.direction * t
    }

    // pub fn hit_sphere(&self, center: Point3, radius: f64) -> bool {
    //     let oc = self.origin - center;
    //     let a = self.direction.dot(self.direction);
    //     let b = 2.0 * self.direction.dot(oc);
    //     let c = oc.dot(oc) - radius.powi(2);
    //     let d = b * b - 4.0 * a * c;
    //     d > 0.0
    // }

    pub fn hit_sphere(&self, center: Point3, radius: f64) -> f64 {
        let oc = self.origin - center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * self.direction.dot(oc);
        let c = oc.dot(oc) - radius.powi(2);
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            -1.0
        } else {
            return (-b - d.sqrt()) / (2.0 * a);
        }
    }
}
