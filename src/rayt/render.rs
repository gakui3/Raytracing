use super::float3::Float3;
use super::ray::Ray;
use super::shape::HitInfo;

pub struct ScatterInfo {
    pub ray: Ray,
    pub albedo: Float3,
}

impl ScatterInfo {
    fn new(ray: Ray, albedo: Float3) -> Self {
        Self { ray, albedo }
    }
}

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Float3,
}

impl Lambertian {
    pub fn new(albedo: Float3) -> Self {
        Self { albedo }
    }
}

pub trait Material: std::fmt::Debug {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
    fn box_clone(&self) -> Box<dyn Material>;
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let target = hit.p + hit.n + Float3::random_in_unit_sphere();
        Some(ScatterInfo::new(
            Ray::new(hit.p, target - hit.p),
            self.albedo,
        ))
    }

    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}
