use image::ImageResult;

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    x: f32,
    y: f32,
    z: f32,
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

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b}
    } 
    pub fn to_bytes(&self) -> (u8, u8, u8) {
        let r = (self.r * 255.) as u8;
        let g = (self.g * 255.) as u8;
        let b = (self.b * 255.) as u8;
        (r, g, b)
    }
}

pub struct Image {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn constant(color: Color, width: usize, height: usize) -> Image {
        let mut pixels = Vec::with_capacity(width*height);
        for _ in 0..width {
            for _ in 0..height {
                pixels.push(color.clone());
            }
        }

        Image { pixels, width, height } 
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.pixels.iter()
            .map(|color|{
                let col = color.to_bytes();
                vec![col.0, col.1, col.2]
            })
            .flatten()
            .collect()
    }

    pub fn save(&self, path: &str) -> ImageResult<()>{
        image::save_buffer(path, self.to_bytes().as_slice(), self.width as u32, self.height as u32, image::ColorType::Rgb8)
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut Color> {
        self.pixels.get_mut(j * self.height + i)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}