use std::fmt::Debug;

use super::float3::Float3;
use super::ray::Ray;

#[derive(Debug)]
pub struct HitInfo {
    pub t: f64,
    pub p: Float3,
    pub n: Float3,
}

impl HitInfo {
    const fn new(t: f64, p: Float3, n: Float3) -> Self {
        Self { t, p, n }
    }
}

pub trait Shape: Debug {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo>;
}

#[derive(Debug)]
pub struct Sphere {
    center: Float3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Float3, radius: f64) -> Sphere {
        Self { center, radius }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius.powi(2);
        let d = b * b - 4.0 * a * c;
        if d < 0.0 {
            return None;
        }
        let t = (-b - d.sqrt()) / (2.0 * a);
        if t < t_min || t_max < t {
            return None;
        }
        let p = ray.at(t);
        let n = (p - self.center) / self.radius;
        Some(HitInfo::new(t, p, n))
    }
}

#[derive(Debug)]
pub struct ShapeList {
    objects: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    pub fn new() -> ShapeList {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn push(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }
}

impl Shape for ShapeList {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let mut hit_info: Option<HitInfo> = None;
        let mut closest_so_far = t1;
        for object in &self.objects {
            if let Some(info) = object.hit(ray, t0, closest_so_far) {
                closest_so_far = info.t;
                hit_info = Some(info);
            }
        }
        hit_info
    }
}

pub struct SimpleScene {
    world: ShapeList,
}

impl SimpleScene {
    pub fn new() -> Self {
        let mut world = ShapeList::new();
        world.push(Box::new(Sphere::new(Float3::new(0.0, 0.0, 1.0), 0.5)));
        world.push(Box::new(Sphere::new(Float3::new(0.0, -100.5, 1.0), 100.0)));
        Self { world }
    }

    pub fn trace(&self, ray: Ray) -> Float3 {
        let hit_info = self.world.hit(&ray, 0.0, f64::MAX);
        if let Some(hit) = hit_info {
            let target = hit.p + hit.n + Float3::random_in_unit_sphere();
            self.trace(Ray::new(hit.p, target - hit.p)) * 0.5
            // (hit.n + Float3::one()) * 0.5
        } else {
            Float3::new(1.0, 1.0, 1.0)
        }
    }
}
