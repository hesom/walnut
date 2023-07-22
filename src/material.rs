use crate::sensor::Color;
use crate::geometry::*;

pub struct BsdfSample {
    pub radiance: Color,
    pub weight: f32,
}

pub trait Bsdf : Send + Sync {
    fn eval(&self, si: &SurfaceInteraction, wo: Vector) -> BsdfSample;
}

pub struct BlackBody {}

pub struct PhongMaterial {
    pub diffuse: Color,
    pub specular: Color,
    pub exponent: f32,
}

impl Bsdf for BlackBody {
    fn eval(&self, _si: &SurfaceInteraction, _wo: Vector) -> BsdfSample {
        BsdfSample {
            radiance: Color::new(0.0, 0.0, 0.0),
            weight: 1.0,
        }
    }
}

impl Bsdf for PhongMaterial {
    fn eval(&self, si: &SurfaceInteraction, wo: Vector) -> BsdfSample {
        let n = si.normal;
        let r_v = -reflect(si.wi, n);
        let diffuse = f32::max(dot(n, wo), 0.0) * self.diffuse;
        let specular = f32::powf(f32::max(dot(r_v, wo), 0.0), self.exponent) * self.specular;

        BsdfSample {
            radiance: diffuse + specular,
            weight: 1.0
        }
    }
}
