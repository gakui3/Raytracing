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
enum RectAxisType {
    XY,
    XZ,
    YZ,
}

#[derive(Debug)]
pub struct Rect {
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    axis: RectAxisType,
    n: Float3,
    material: Arc<dyn Material>,
}

impl Shape for Rect {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let mut origin = ray.origin;
        let mut direction = ray.direction;
        let mut axis = Float3::zaxis();
        match self.axis {
            RectAxisType::XY => {}
            RectAxisType::XZ => {
                origin = Float3::new(origin.x(), origin.z(), origin.y());
                direction = Float3::new(direction.x(), direction.z(), direction.y());
                axis = Float3::yaxis();
            }
            RectAxisType::YZ => {
                origin = Float3::new(origin.y(), origin.z(), origin.x());
                direction = Float3::new(direction.y(), direction.z(), direction.x());
                axis = Float3::xaxis();
            }
        }
        let t = (self.k - origin.z()) / direction.z();
        if t < t0 || t > t1 {
            return None;
        }
        let x = origin.x() + t * direction.x();
        let y = origin.y() + t * direction.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        Some(HitInfo::new(
            t,
            ray.at(t),
            self.n,
            Arc::clone(&self.material),
        ))
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
        // world.push(Box::new(Sphere::new(
        //     Float3::new(0.0, 0.0, 1.0),
        //     0.5,
        //     Arc::new(Lambertian::new(Float3::new(0.8, 0.3, 0.3))),
        // )));

        // world.push(Box::new(Sphere::new(
        //     Float3::new(0.0, 0.0, 1.0),
        //     0.25,
        //     // Arc::new(Metal::new(Float3::new(0.3, 0.8, 0.0), 0.0)),
        //     // Arc::new(Lambertian::new(Float3::new(0.8, 0.3, 0.3))),
        //     Arc::new(DiffuseLight::new(Float3::new(10.0, 10.0, 10.0))),
        // )));

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

        world.push(Box::new(Box3D::new(
            Float3::new(130.0, 0.0, 65.0),
            Float3::new(295.0, 165.0, 230.0),
            Arc::new(Lambertian::new(Float3::new(0.73, 0.73, 0.73))),
        )));

        //コーネルボックスを作成
        //左の壁
        world.push(Box::new(Rect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 555.0,
            axis: RectAxisType::YZ,
            n: Float3::new(-1.0, 0.0, 0.0),
            material: Arc::new(Lambertian::new(Float3::new(0.12, 0.45, 0.15))),
        }));
        //右の壁
        world.push(Box::new(Rect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 0.0,
            axis: RectAxisType::YZ,
            n: Float3::new(1.0, 0.0, 0.0),
            material: Arc::new(Lambertian::new(Float3::new(0.65, 0.05, 0.05))),
        }));
        //天井のライト
        world.push(Box::new(Rect {
            x0: 213.0,
            x1: 343.0,
            y0: 227.0,
            y1: 332.0,
            k: 554.0,
            axis: RectAxisType::XZ,
            n: Float3::new(0.0, -1.0, 0.0),
            material: Arc::new(DiffuseLight::new(Float3::new(40.0, 40.0, 40.0))),
        }));
        //天井
        world.push(Box::new(Rect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 555.0,
            axis: RectAxisType::XZ,
            n: Float3::new(0.0, -1.0, 0.0),
            material: Arc::new(Lambertian::new(Float3::new(0.73, 0.73, 0.73))),
        }));
        //奥の壁
        world.push(Box::new(Rect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 555.0,
            axis: RectAxisType::XY,
            n: Float3::new(0.0, 0.0, -1.0),
            material: Arc::new(Lambertian::new(Float3::new(0.73, 0.73, 0.73))),
        }));
        //床
        world.push(Box::new(Rect {
            x0: 0.0,
            x1: 555.0,
            y0: 0.0,
            y1: 555.0,
            k: 0.0,
            axis: RectAxisType::XZ,
            n: Float3::new(0.0, 0.0, 1.0),
            material: Arc::new(Lambertian::new(Float3::new(0.73, 0.73, 0.73))),
        }));

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

#[derive(Debug)]
pub struct Box3D {
    p0: Float3,
    p1: Float3,
    shapes: ShapeList,
}
impl Box3D {
    fn new(p0: Float3, p1: Float3, material: Arc<dyn Material>) -> Self {
        let mut shapes = ShapeList::new();
        shapes.push(Box::new(Rect {
            x0: p0.x(),
            x1: p1.x(),
            y0: p0.y(),
            y1: p1.y(),
            k: p1.z(),
            axis: RectAxisType::XY,
            n: Float3::new(1.0, 0.0, 0.0),
            material: Arc::clone(&material),
        }));
        shapes.push(Box::new(Rect {
            x0: p0.x(),
            x1: p1.x(),
            y0: p0.y(),
            y1: p1.y(),
            k: p0.z(),
            axis: RectAxisType::XY,
            n: Float3::new(-1.0, 0.0, 0.0),
            material: Arc::clone(&material),
        }));
        shapes.push(Box::new(Rect {
            x0: p0.x(),
            x1: p1.x(),
            y0: p0.z(),
            y1: p1.z(),
            k: p1.y(),
            axis: RectAxisType::XZ,
            n: Float3::new(0.0, 1.0, 0.0),
            material: Arc::clone(&material),
        }));
        shapes.push(Box::new(Rect {
            x0: p0.x(),
            x1: p1.x(),
            y0: p0.z(),
            y1: p1.z(),
            k: p0.y(),
            axis: RectAxisType::XZ,
            n: Float3::new(0.0, -1.0, 0.0),
            material: Arc::clone(&material),
        }));
        // shapes.push(
        //     ShapeBuilder::new()
        //         .material(Arc::clone(&material))
        //         .rect_xz(p0.x(), p1.x(), p0.z(), p1.z(), p1.y())
        //         .build(),
        // );
        // shapes.push(
        //     ShapeBuilder::new()
        //         .material(Arc::clone(&material))
        //         .rect_xz(p0.x(), p1.x(), p0.z(), p1.z(), p0.y())
        //         .flip_face()
        //         .build(),
        // );
        // shapes.push(
        //     ShapeBuilder::new()
        //         .material(Arc::clone(&material))
        //         .rect_yz(p0.y(), p1.y(), p0.z(), p1.z(), p1.x())
        //         .build(),
        // );
        // shapes.push(
        //     ShapeBuilder::new()
        //         .material(Arc::clone(&material))
        //         .rect_yz(p0.y(), p1.y(), p0.z(), p1.z(), p0.x())
        //         .flip_face()
        //         .build(),
        // );
        Self { p0, p1, shapes }
    }
}
impl Shape for Box3D {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        self.shapes.hit(ray, t0, t1)
    }
}
