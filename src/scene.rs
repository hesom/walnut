use crate::material::*;
use crate::math::*;

pub struct SurfaceInteraction<'a> {
    pub position: Point,
    pub normal: Vector,
    pub material: &'a Box<dyn Bsdf>,
    pub wi: Vector,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Box<dyn Bsdf>,
}

pub trait Shape : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
}

pub struct Scene {
    pub shapes: Vec<Box<dyn Shape>>,
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
        })
    }

    pub fn new() -> Scene {
        Scene { shapes: Vec::new() }
    }

    pub fn add(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: Box<dyn Bsdf>) -> Sphere {
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
            material: &self.material
        })
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
