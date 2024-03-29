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

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Float3,
    fuzz: f64,
}

impl Lambertian {
    pub fn new(albedo: Float3) -> Self {
        Self { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Float3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

pub trait Material: std::fmt::Debug + Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
    fn emitted(&self, _ray: &Ray, _hit: &HitInfo) -> Float3 {
        Float3::zero()
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let target = hit.p + hit.n + Float3::randpm_unit_vector();
        Some(ScatterInfo::new(
            Ray::new(hit.p, target - hit.p),
            self.albedo,
        ))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let mut reflected = ray.direction.normalize().reflect(hit.n);
        reflected = reflected + Float3::randpm_unit_vector() * self.fuzz;
        if reflected.dot(hit.n) > 0.0 {
            Some(ScatterInfo::new(Ray::new(hit.p, reflected), self.albedo))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiffuseLight {
    emit: Float3,
}
impl DiffuseLight {
    pub fn new(emit: Float3) -> Self {
        Self { emit }
    }
}
impl Material for DiffuseLight {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        None
    }
    fn emitted(&self, ray: &Ray, hit: &HitInfo) -> Float3 {
        self.emit
    }
}
