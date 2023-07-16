use walnut::*;

fn main() {

    let sensor = Sensor::zero(800, 800);
    let mut camera = PinholeCamera::new(sensor, 45.0);

    let sphere = Sphere{
        center: Point { x: 0.0, y: 0.0, z: -5.0 },
        radius: 1.0,
    };

    for i in 0..800 {
        for j in 0..800 {
            let Some(ray) = camera.sample_ray(i, j) else {
                continue
            };
            
            let Some(si) = sphere.intersect(&ray) else {
                continue
            };

            if let Some(pixel) = camera.get_sensor_mut().get_mut(i, j){
                let n = si.normal;
                *pixel = Color::new(n.x, n.y, n.z);
            }
        }
    }

    camera.get_sensor_mut().save("image.png").expect("Error writing file");
}