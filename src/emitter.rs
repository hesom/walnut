use crate::geometry::*;
use crate::sensor::Color;

pub trait Emitter {
    fn sample(&self) -> EmitterSample;
}

pub struct EmitterSample {
    pub radiance: Color,
    pub position: Point,
    pub weight: f32,
}

pub struct PointLight {
    position: Point,
    intensity: Color,
}

impl Emitter for PointLight {
    fn sample(&self) -> EmitterSample {
        EmitterSample {
            radiance: self.intensity,
            position: self.position,
            weight: 1.0
        }
    }
}

impl PointLight {
    pub fn new(position: Point, intensity: f32) -> PointLight {
        PointLight {
            position,
            intensity: Color::new(intensity, intensity, intensity),
        }
    }

    pub fn new_colored(position: Point, color: Color) -> PointLight {
        PointLight {
            position,
            intensity: color,
        }
    }
}
