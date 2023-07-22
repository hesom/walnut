use std::ops::{Add, Mul, Neg, Sub};
use crate::material::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

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

impl Vector {
    pub fn normalize(&self) -> Vector {
        let length = f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for Point {
    type Output = Point;
    fn neg(self) -> Self::Output {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub fn dot(a: Vector, b: Vector) -> f32 {
    a.x * b.x + a.y * b.y + a.z * b.z
}

pub fn norm(a: Vector) -> f32 {
    f32::sqrt(norm2(a))
}

pub fn norm2(a: Vector) -> f32 {
    dot(a, a)
}

pub fn reflect(a: Vector, n: Vector) -> Vector {
    a - 2.0*dot(a, n) * n
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
    fn it_adds() {
        let v = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let w = Vector {
            x: 0.0,
            y: 1.0,
            z: -3.0,
        };
        let res = v + w;
        assert_eq!(
            res,
            Vector {
                x: 1.0,
                y: 3.0,
                z: 0.0
            }
        );

        let p = Point {
            x: 0.0,
            y: 1.0,
            z: -3.0,
        };
        let res = v + p;
        assert_eq!(
            res,
            Point {
                x: 1.0,
                y: 3.0,
                z: 0.0
            }
        );

        let res = p + v;
        assert_eq!(
            res,
            Point {
                x: 1.0,
                y: 3.0,
                z: 0.0
            }
        );
    }

    #[test]
    fn it_subtracts() {
        let v = Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let w = Vector {
            x: 0.0,
            y: 1.0,
            z: -3.0,
        };
        let res = v - w;
        assert_eq!(
            res,
            Vector {
                x: 1.0,
                y: 1.0,
                z: 6.0
            }
        );
    }

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
