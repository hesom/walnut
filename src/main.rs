use walnut::*;

fn main() {
    let spp = 255;
    let sensor = Sensor::zero(800, 800);
    let camera = PinholeCamera::new(sensor, 45.0);

    let sphere = Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -5.0,
        },
        radius: 1.0,
    };

    for pixel in camera.get_pixels().iter() {
        let (i, j) = pixel.position;

        let n = (0..spp)
            .into_iter()
            .filter_map(|_| camera.sample_ray(i, j))
            .filter_map(|ray| sphere.intersect(&ray))
            .map(|si| si.normal)
            .reduce(|accum, normal| accum + normal);

        if let Some(n) = n {
            let f = 1.0 / spp as f32;
            pixel.color.set(Color::new(f*n.x, f*n.y, f*n.z));
        }
    }

    camera
        .get_sensor()
        .save("image.png")
        .expect("Error writing file");
}
