mod sensor;
pub use sensor::{Sensor, Color};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector {
    fn normalize(&self) -> Vector {
        let length = f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

pub struct SurfaceInteraction {
    position: Point,
}

pub trait Shape {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
}

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        return None
    }
}