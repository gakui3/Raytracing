use super::float3::{Point3, Vector3};
use super::ray::Ray;

pub struct Camera {
    pub origin: Point3,
    pub u: Vector3,
    pub v: Vector3,
    pub w: Vector3,
}

impl Camera {
    pub fn new(origin: Point3, u: Vector3, v: Vector3, w: Vector3) -> Self {
        Self { origin, u, v, w }
    }

    pub fn from_lookat(
        origin: Vector3,
        lookat: Vector3,
        vup: Vector3,
        vfov: f64,
        aspect: f64,
    ) -> Self {
        let halfh = (vfov.to_radians() * 0.5).tan();
        let halfw = aspect * halfh;
        let w = (origin + lookat).normalize(); //基底ベクトル
        let x = vup.cross(w).normalize(); //基底ベクトル
        let y = w.cross(x).normalize(); //基底ベクトル
        let xw = x * halfw * 2.0;
        let yh = y * halfh * 2.0;
        Self {
            origin,
            u: xw * 2.0,
            v: yh * 2.0,
            w: origin - xw - yh + w,
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.w + self.u * u + self.v * v - self.origin,
        }
    }
}
