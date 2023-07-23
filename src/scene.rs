use crate::material::*;
use crate::math::*;
use crate::sensor::Color;
use crate::emitter::Emitter;

pub struct SurfaceInteraction<'a> {
    pub position: Point,
    pub normal: Vector,
    pub material: &'a Box<dyn Material>,
    pub wi: Vector,
    pub emitter: Option<&'a Box<dyn Emitter>>,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Box<dyn Material>,
}

pub trait Shape : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
}

pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
    pub lights: Vec<Box<dyn Emitter>>,
    pub background_color: Color,
}

impl Scene {
    pub fn closest_hit(&self, ray: &Ray) -> Option<SurfaceInteraction> {
       let mut hits : Vec<_> = self.shapes.iter()
           .filter_map(|shape| shape.intersect(&ray))
           .collect();
      
        if hits.is_empty() {
            return None
        }

        hits.sort_by(|a, b| {
            let dist1 = norm2(ray.origin - a.position);
            let dist2 = norm2(ray.origin - b.position);
            dist1.partial_cmp(&dist2).unwrap()
        });

        let closest = hits.get(0)?;

        Some(SurfaceInteraction {
            position: closest.position,
            normal: closest.normal,
            material: closest.material,
            wi: closest.wi,
            emitter: None,
        })
    }

    pub fn new() -> Scene {
        Scene {
            shapes: Vec::new(),
            lights: Vec::new(),
            background_color: Color::new(0.2, 0.2, 0.2)
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
        Sphere { center, radius, material }
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
            return None
        }

        let intersection = o + t * u;
        let normal = (intersection - c).normalize();

        Some(SurfaceInteraction {
            position: intersection,
            normal,
            wi: -u,
            material: &self.material,
            emitter: None,
        })
    }
}

impl<'a> SurfaceInteraction<'a> {
    pub fn to_local_frame(&self, v: Vector) -> Vector {
        let w = self.normal;
        let u = cross(Vector{ x: 0.0, y: 1.0, z: 0.0}, w).normalize();
        let vv = cross(w, u);

        Vector {
            x: dot(vv, v),
            y: dot(u, v),
            z: dot(w, v),
        }
    }

    pub fn to_global_frame(&self, v: Vector) -> Vector {
        let w = self.normal;
        let u = cross(Vector{ x: 0.0, y: 1.0, z: 0.0}, w).normalize();
        let vv = cross(w, u);

        Vector {
            x: v.x * u.x + v.y * vv.x + v.z * w.x,
            y: v.x * u.y + v.y * vv.y + v.z * w.y,
            z: v.x * u.z + v.y * vv.z + v.z * w.z,
        }
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

        let sphere = Sphere { center, radius, material: Box::new(BlackBody{}) };

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
