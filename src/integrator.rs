use crate::math::*;
use crate::scene::*;
use crate::sensor::Color;
use crate::material::*;

use rand::Rng;

pub trait Integrator: Send + Sync {
    fn sample_radiance(&self, ray: &Ray, scene: &Scene) -> Color;
}

pub struct PathIntegrator {
    max_bounce: usize,
    russian_roulette: usize,
}

impl PathIntegrator {
    pub fn new(max_bounce: usize, russian_roulette: usize) -> PathIntegrator {
        PathIntegrator { max_bounce , russian_roulette }
    }
}

impl Integrator for PathIntegrator {
    fn sample_radiance(&self, ray: &Ray, scene: &Scene) -> Color {
        let mut throughput = Color::new(1.0, 1.0, 1.0);
        let mut color = Color::new(0.0, 0.0, 0.0);

        let mut ray = Ray {
            origin: ray.origin,
            direction: ray.direction,
        };

        let mut rng = rand::thread_rng();

        for bounce in 0..self.max_bounce {
            let Some(si) = scene.closest_hit(&ray) else {
                color = color + throughput * scene.background_color;
                break;
            };

            if let Some(light) = si.emitter {
                color = color + throughput * light.sample().radiance;
            }

            let mut le = Color::new(0.0, 0.0, 0.0);
            for light in scene.lights.iter() {
                let light_sample = light.sample();
                let wo = (light_sample.position - si.position).normalize();
                let shadow_si = scene.closest_hit(&Ray {
                    origin: si.position + 1e-3*wo,
                    direction: wo,
                });
                if let Some(_) = shadow_si {
                    continue;
                }

                le = le + si.material.bsdf_eval(&si, wo).radiance * light_sample.radiance;
            }

            color = color + throughput * le;

            // compute new ray direction
            let wo = si.material.bsdf_sample(&si);
            let BsdfSample{radiance, pdf} = si.material.bsdf_eval(&si, wo);

            throughput = (1.0/pdf) * throughput * radiance;
            
            ray.origin = si.position + 1e-3*wo;
            ray.direction = wo;

            if bounce > self.russian_roulette {
                let p = f32::max(throughput.r, f32::max(throughput.g, throughput.b));
                if rng.gen::<f32>() > p {
                    break;
                }
                throughput = (1.0 / p) * throughput;
            }
        }

        color
    }
}
