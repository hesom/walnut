use crate::emitter::Emitter;
use crate::material::*;
use crate::math::*;
use crate::sensor::Color;

pub struct SurfaceInteraction<'a> {
    pub position: Point,
    pub normal: Vector,
    pub t: f32,
    pub material: &'a Box<dyn Material>,
    pub wi: Vector,
    pub emitter: Option<&'a Box<dyn Emitter>>,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

pub struct InfinitePlane {
    pub center: Point,
    pub normal: Vector,
    pub material: Box<dyn Material>,
}

pub trait Shape: Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
}

pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
    pub lights: Vec<Box<dyn Emitter>>,
    pub background_color: Color,
}

impl Scene {
    pub fn closest_hit(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let closest = self
            .shapes
            .iter()
            .filter_map(|shape| shape.intersect(&ray))
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())?;

        Some(SurfaceInteraction {
            position: closest.position,
            normal: closest.normal,
            t: closest.t,
            material: closest.material,
            wi: closest.wi,
            emitter: None,
        })
    }

    pub fn new() -> Scene {
        Scene {
            shapes: Vec::new(),
            lights: Vec::new(),
            background_color: Color::new(0.2, 0.2, 0.2),
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn add_light(&mut self, light: Box<dyn Emitter>) {
        self.lights.push(light);
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl InfinitePlane {
    pub fn new(center: Point, normal: Vector, material: Box<dyn Material>) -> InfinitePlane {
        InfinitePlane {
            center,
            normal,
            material,
        }
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let o = ray.origin;
        let u = ray.direction.normalize();
        let c = self.center;
        let r = self.radius;

        let discriminant = f32::powi(dot(u, o - c), 2) - (norm2(o - c) - r * r);

        if discriminant < 0.0 {
            return None;
        }

        let t = -dot(u, o - c) - f32::sqrt(discriminant);
        if t < 0.0 {
            return None;
        }

        let intersection = o + t * u;
        let normal = (intersection - c).normalize();

        Some(SurfaceInteraction {
            position: intersection,
            normal,
            t,
            wi: -u,
            material: &self.material,
            emitter: None,
        })
    }
}

impl Shape for InfinitePlane {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let o = ray.origin;
        let u = ray.direction.normalize();
        let n = self.normal;
        let c = self.center;

        let denom = dot(u, n);
        if denom > -1e-6 {
            return None;
        }

        let t = dot(c - o, n) / denom;

        if t < 0.0 {
            return None;
        }

        let intersection = o + t * u;

        Some(SurfaceInteraction {
            position: intersection,
            normal: n,
            t,
            wi: -u,
            material: &self.material,
            emitter: None,
        })
    }
}

impl<'a> SurfaceInteraction<'a> {
    pub fn local_frame(&self) -> (Vector, Vector, Vector) {
        let w = self.normal;
        let axis = match f32::abs(w.x) > 0.1 {
            true => Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            false => Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };
        let u = cross(axis, w).normalize();
        let v = cross(w, u);

        (u, v, w)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_intersects() {
        let center = Point {
            x: 0.0,
            y: 5.0,
            z: 0.0,
        };
        let radius = 3.0;

        let sphere = Sphere {
            center,
            radius,
            material: Box::new(BlackBody {}),
        };

        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            direction: Vector {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        };

        let si = sphere.intersect(&ray);
        assert!(si.is_some());
        let position = si.unwrap().position;

        assert_eq!(
            position,
            Point {
                x: 0.0,
                y: 2.0,
                z: 0.0
            }
        );

        let ray = Ray {
            origin: Point {
                x: 0.0,
                y: -1.0,
                z: 0.0,
            },
            direction: Vector {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let si = sphere.intersect(&ray);

        assert!(si.is_none());
    }
}
