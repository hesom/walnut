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
            match camera.sample_ray(i, j) {
                Some(ray) => {
                    match sphere.intersect(&ray) {
                        Some(si) => {
                            if let Some(pixel) = camera.get_sensor_mut().get_mut(i, j){
                                let n = si.normal;
                                *pixel = Color::new(n.x, n.y, n.z);
                            }
                        },
                        None => continue,
                    }
                },
                None => continue,
            };
        }
    }

    camera.get_sensor_mut().save("image.png").expect("Error writing file");
}