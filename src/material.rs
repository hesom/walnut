use crate::sensor::Color;
use crate::scene::*;
use crate::math::*;

pub struct BsdfSample {
    pub radiance: Color,
    pub weight: f32,
}

pub trait Material : Send + Sync {
    fn bsdf(&self, si: &SurfaceInteraction, wo: Vector) -> BsdfSample;
}

pub struct BlackBody {}

pub struct PhongMaterial {
    pub diffuse: Color,
    pub specular: Color,
    pub exponent: f32,
}

impl Material for BlackBody {
    fn bsdf(&self, _si: &SurfaceInteraction, _wo: Vector) -> BsdfSample {
        BsdfSample {
            radiance: Color::new(0.0, 0.0, 0.0),
            weight: 1.0,
        }
    }
}

impl Material for PhongMaterial {
    fn bsdf(&self, si: &SurfaceInteraction, wo: Vector) -> BsdfSample {
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
