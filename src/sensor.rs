use std::ops::{Add, Mul};
use image::ImageResult;
use rand::Rng;
use std::sync::RwLock;

use crate::math::*;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

pub struct Pixel {
    pub position: (usize, usize),
    pub color: RwLock<Color>,
}

pub struct Sensor {
    pixels: Vec<Pixel>,
    width: usize,
    height: usize,
}

pub struct PinholeCamera {
    sensor: Sensor,
    fov: f32,
    position: Point,
}

pub trait Camera : Send + Sync {
    fn get_sensor_mut(&mut self) -> &mut Sensor;
    fn get_sensor(&self) -> &Sensor;
    fn get_pixels_mut(&mut self) -> &mut Vec<Pixel>;
    fn get_pixels(&self) -> &Vec<Pixel>;
    fn sample_ray(&self, i: usize, j: usize) -> Option<Ray>;
}

impl PinholeCamera {
    pub fn new(sensor: Sensor, fov: f32) -> PinholeCamera {
        PinholeCamera {
            sensor,
            fov: fov.to_radians(),
            position: Point {x: 0.0, y:0.0, z: 0.0 },
        }
    }

    pub fn position(&self) -> Point {
        self.position
    }
}

impl Camera for PinholeCamera {
    fn get_sensor_mut(&mut self) -> &mut Sensor {
        &mut self.sensor
    }

    fn get_sensor(&self) -> &Sensor {
        &self.sensor
    }

    fn get_pixels_mut(&mut self) -> &mut Vec<Pixel> {
        &mut self.sensor.pixels
    }

    fn get_pixels(&self) -> &Vec<Pixel> {
        &self.sensor.pixels
    }

    fn sample_ray(&self, i: usize, j: usize) -> Option<Ray> {
        if !self.sensor.inside(i, j) {
            return None;
        }

        let aspect_ratio = self.sensor.aspect();

        let mut rng = rand::thread_rng();
        let jitter_u: f32 = rng.gen();
        let jitter_v: f32 = rng.gen();

        // pixel coord to normalized coord in [0, 1]
        let u = (i as f32 + jitter_u) / (self.sensor.width + 1) as f32;
        let v = (j as f32 + jitter_v) / (self.sensor.height + 1) as f32;

        let u = (2.0 * u - 1.0) * aspect_ratio * f32::tan(self.fov / 2.0);
        let v = (1.0 - 2.0 * v) * f32::tan(self.fov / 2.0);

        Some(Ray {
            origin: Point {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            direction: Vector {
                x: u,
                y: v,
                z: -1.0,
            }
            .normalize(),
        })
    }
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }
    pub fn to_bytes(&self) -> (u8, u8, u8) {
        let r = (self.r * 255.) as u8;
        let g = (self.g * 255.) as u8;
        let b = (self.b * 255.) as u8;
        (r, g, b)
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: f32::clamp(self.r, 0.0, 1.0),
            g: f32::clamp(self.g, 0.0, 1.0),
            b: f32::clamp(self.b, 0.0, 1.0),
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Sensor {
    pub fn constant(color: Color, width: usize, height: usize) -> Sensor {
        let mut pixels = Vec::with_capacity(width * height);
        for j in 0..height {
            for i in 0..width {
                let pixel = Pixel {
                    position: (i, j),
                    color: RwLock::new(color.clone()),
                };
                pixels.push(pixel);
            }
        }

        Sensor {
            pixels,
            width,
            height,
        }
    }

    pub fn zero(width: usize, height: usize) -> Sensor {
        Sensor::constant(
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            },
            width,
            height,
        )
    }

    pub fn clear(&self) {
        for pixel in self.pixels.iter() {
            *pixel.color.write().unwrap() = Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
        }
    }

    pub fn readout(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .map(|Pixel { color, .. }| {
                let col = color.read().unwrap().to_bytes();
                vec![col.0, col.1, col.2]
            })
            .flatten()
            .collect()
    }

    pub fn save(&self, path: &str) -> ImageResult<()> {
        image::save_buffer(
            path,
            self.readout().as_slice(),
            self.width as u32,
            self.height as u32,
            image::ColorType::Rgb8,
        )
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Pixel> {
        if !self.inside(i, j) {
            return None;
        }
        self.pixels.get_mut(j * self.width + i)
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&Pixel> {
        if !self.inside(i, j) {
            return None;
        }
        self.pixels.get(j * self.width + i)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn inside(&self, i: usize, j: usize) -> bool {
        i < self.width && j < self.height
    }

    pub fn aspect(&self) -> f32 {
        self.width as f32 / self.height as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_outside() {
        let sensor = Sensor::zero(10, 20);
        assert!(!sensor.inside(10, 10));
        assert!(!sensor.inside(5, 20));
        assert!(sensor.inside(5, 10));

        assert!(sensor.get(10, 10).is_none());
        assert!(sensor.get(5, 20).is_none());
        assert!(sensor.get(0, 20).is_none());
        assert!(sensor.get(10, 0).is_none());
        assert!(sensor.get(5, 10).is_some());
        assert!(sensor.get(9, 19).is_some());
        assert!(sensor.get(0, 0).is_some());
        assert!(sensor.get(9, 0).is_some());
        assert!(sensor.get(0, 19).is_some());
    }

    #[test]
    fn clears() {
        let sensor = Sensor::constant(
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
            },
            10,
            20,
        );
        sensor.clear();

        for pixel in sensor.pixels {
            let col = pixel.color.read().unwrap();
            assert_eq!(col.r, 0.0);
            assert_eq!(col.g, 0.0);
            assert_eq!(col.b, 0.0);
        }
    }

    #[test]
    fn projects_correctly() {
        let sensor = Sensor::zero(200, 100);
        let camera = PinholeCamera::new(sensor, 45.0);

        let ray = camera.sample_ray(0, 0).unwrap();

        assert_eq!(
            ray.origin,
            Point {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        );
        assert!(ray.direction.z < 0.0);
    }
}
