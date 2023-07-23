use crate::sensor::Color;
use crate::scene::*;
use crate::math::*;
use rand::Rng;

pub struct BsdfSample {
    pub radiance: Color,
    pub pdf: f32,
}

fn uniform_hemisphere_sample(si: &SurfaceInteraction) -> Vector {
    let (u, v, w) = si.local_frame();

    let mut rng = rand::thread_rng();
    let e1 : f32 = rng.gen();
    let e2 : f32 = rng.gen();

    let r = f32::sqrt(1.0 - e1*e1);
    let phi = 2.0 * std::f32::consts::PI * e2;

    f32::cos(phi) * r * u + f32::sin(phi) * r * v + e1 * w
}

fn cosine_weighted_hemisphere_sample(si: &SurfaceInteraction) -> Vector {
    let (u, v, w) = si.local_frame();

    let mut rng = rand::thread_rng();
    let e1 : f32 = rng.gen();
    let e2 : f32 = rng.gen();

    let r = f32::sqrt(e1);
    let phi = 2.0 * std::f32::consts::PI * e2;

    f32::cos(phi) * r * u + f32::sin(phi) * r * v + f32::sqrt(f32::max(0.0, 1.0 - e1)) * w
}

pub trait Material : Send + Sync {
    fn bsdf_eval(&self, si: &SurfaceInteraction, wo: Vector) -> BsdfSample;
    fn bsdf_sample(&self, si: &SurfaceInteraction) -> Vector;
    fn bsdf_pdf(&self, si: &SurfaceInteraction, wo: Vector) -> f32;
    fn is_delta_reflector(&self) -> bool;
}

pub struct BlackBody {}

pub struct PhongMaterial {
    pub albedo: Color,
    pub specular: Color,
    pub exponent: f32,
}

pub struct DiffuseMaterial {
    pub albedo: Color,
}

impl Material for BlackBody {
    fn bsdf_eval(&self, si: &SurfaceInteraction, wo: Vector) -> BsdfSample {
        BsdfSample {
            radiance: Color::new(0.0, 0.0, 0.0),
            pdf: self.bsdf_pdf(&si, wo),
        }
    }

    fn bsdf_sample(&self, si: &SurfaceInteraction) -> Vector {
        cosine_weighted_hemisphere_sample(&si)
    }

    fn bsdf_pdf(&self, si: &SurfaceInteraction, wo: Vector) -> f32 {
        dot(si.normal, wo) / std::f32::consts::PI
    }

    fn is_delta_reflector(&self) -> bool {
        false
    }
}

impl Material for PhongMaterial {
    fn bsdf_eval(&self, si: &SurfaceInteraction, wo: Vector) -> BsdfSample {
        let n = si.normal;
        let r_v = -reflect(si.wi, n);
        let diffuse = (1.0 / std::f32::consts::PI) * f32::max(dot(n, wo), 0.0) * self.albedo;
        let specular = f32::powf(f32::max(dot(r_v, wo), 0.0), self.exponent) * self.specular;

        BsdfSample {
            radiance: diffuse + specular,
            pdf: self.bsdf_pdf(&si, wo)
        }
    }

    fn bsdf_sample(&self, si: &SurfaceInteraction) -> Vector {
        uniform_hemisphere_sample(&si)
    }

    fn bsdf_pdf(&self, _si: &SurfaceInteraction, _wo: Vector) -> f32 {
        1.0 / (2.0 * std::f32::consts::PI)
    }

    fn is_delta_reflector(&self) -> bool {
        false
    }
}

impl Material for DiffuseMaterial {    
    fn bsdf_eval(&self, si: &SurfaceInteraction, wo: Vector) -> BsdfSample {
        let n = si.normal;
        let diffuse = (1.0 / std::f32::consts::PI) * f32::max(dot(n, wo), 0.0) * self.albedo;

        BsdfSample { radiance: diffuse, pdf: self.bsdf_pdf(&si, wo) }
    }

    fn bsdf_sample(&self, si: &SurfaceInteraction) -> Vector {
        cosine_weighted_hemisphere_sample(&si)
    }

    fn bsdf_pdf(&self, si: &SurfaceInteraction, wo: Vector) -> f32 {
        dot(si.normal, wo) / std::f32::consts::PI
    }

    fn is_delta_reflector(&self) -> bool {
        false
    }
}