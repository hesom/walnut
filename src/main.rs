use walnut::*;

fn main() {

    let sensor = Sensor::zero(800, 800);
    let camera = PinholeCamera::new(sensor, 45.0);

    let sphere = Sphere{
        center: Point { x: 0.0, y: 0.0, z: -5.0 },
        radius: 1.0,
    };

    for pixel in camera.get_pixels().iter() {
       let (i, j) = pixel.position;

       let Some(ray) = camera.sample_ray(i, j) else {
           continue;
       };

       let Some(si) = sphere.intersect(&ray) else {
           continue;
       };

       let n = si.normal;
       pixel.color.set(Color::new(n.x, n.y, n.z));
    }

    camera.get_sensor().save("image.png").expect("Error writing file");
}
