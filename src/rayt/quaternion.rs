use super::float3::Vector3;

pub struct Quaternion(pub Vector3, pub f64);

impl Quaternion {
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self(Vector3::new(x, y, z), w)
    }
}
