use std::fmt::Debug;
use std::sync::Arc;

use super::float3::Float3;
use super::ray::Ray;
use super::render::{DiffuseLight, Lambertian, Material, Metal};

use rand::prelude::*;

#[derive(Debug)]
pub struct HitInfo {
    pub t: f64,
    pub p: Float3,
    pub n: Float3,
    pub m: Arc<dyn Material>,
}

impl HitInfo {
    fn new(t: f64, p: Float3, n: Float3, m: Arc<dyn Material>) -> Self {
        Self { t, p, n, m }
    }
}

pub trait Shape: Sync + Debug {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo>;
}

#[derive(Debug)]
pub struct Sphere {
    center: Float3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Float3, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius.powi(2);
        let d = b * b - 4.0 * a * c;
        if d > 0.0 {
            let root = d.sqrt();
            let temp = (-b - root) / (2.0 * a);
            if t_min < temp && temp < t_max {
                let p = ray.at(temp);
                return Some(HitInfo::new(
                    temp,
                    p,
                    (p - self.center) / self.radius,
                    Arc::clone(&self.material),
                ));
            }
            let temp = (-b + root) / (2.0 * a);
            if t_min < temp && temp < t_max {
                let p = ray.at(temp);
                return Some(HitInfo::new(
                    temp,
                    p,
                    (p - self.center) / self.radius,
                    Arc::clone(&self.material),
                ));
            }
        }
        None
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
        world.push(Box::new(Sphere::new(
            Float3::new(-0.75, 0.0, 1.0),
            0.5,
            Arc::new(Lambertian::new(Float3::new(0.8, 0.3, 0.3))),
        )));

        world.push(Box::new(Sphere::new(
            Float3::new(0.75, 0.0, 1.0),
            0.5,
            // Arc::new(Metal::new(Float3::new(0.3, 0.8, 0.0), 0.0)),
            // Arc::new(Lambertian::new(Float3::new(0.8, 0.3, 0.3))),
            Arc::new(DiffuseLight::new(Float3::new(10.0, 10.0, 10.0))),
        )));

        // for _ in 0..5 {
        //     world.push(Box::new(Sphere::new(
        //         Float3::new(
        //             random::<f64>() * 10.0 - 5.0,
        //             0.0,
        //             random::<f64>() * 5.0 + 1.0,
        //         ), // 第一引数のFloat3は完全にランダム
        //         0.5,
        //         Arc::new(Lambertian::new(Float3::new(
        //             random::<f64>(),
        //             random::<f64>(),
        //             random::<f64>(),
        //         ))),
        //     )));
        // }

        world.push(Box::new(Sphere::new(
            Float3::new(0.0, -1000.5, 0.0),
            1000.0,
            Arc::new(Lambertian::new(Float3::new(0.3, 0.3, 0.3))),
        )));

        Self { world }
    }

    pub fn trace(&self, ray: Ray, depth: usize) -> Float3 {
        let hit_info = self.world.hit(&ray, 0.001, f64::MAX);
        if let Some(hit) = hit_info {
            let emitted = hit.m.emitted(&ray, &hit);
            let scatter_info = if depth > 0 {
                hit.m.scatter(&ray, &hit)
            } else {
                None
            };
            if let Some(scatter) = scatter_info {
                return self.trace(scatter.ray, depth - 1) * scatter.albedo + emitted;
            } else {
                return emitted;
            }
        } else {
            Float3::new(0.0, 0.0, 0.0)
        }
    }
}
