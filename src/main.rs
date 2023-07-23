use walnut::*;

use std::thread;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let spp = 1024;
    let sensor = Sensor::zero(800, 800);
    let camera = Arc::new(PinholeCamera::new(sensor, 75.0));
    let integrator = Arc::new(PathIntegrator::new(16, 4));

    let mut scene = Scene::new();

    let sphere1 = Sphere::new(
        Point {
            x: 0.0,
            y: 0.0,
            z: -2.5,
        },
        1.0,
        Box::new(PhongMaterial{
            albedo: Color::new(0.8, 0.0, 0.0),
            specular: Color::new(0.8, 0.8, 0.8),
            exponent: 10.0,
        }),
    );

    let sphere2 = Sphere::new(
        Point {
            x: 0.5,
            y: 0.5,
            z: -1.0,
        },
        0.1,
        Box::new(PhongMaterial{
            albedo: Color::new(0.0, 0.2, 0.8),
            specular: Color::new(1.0, 1.0, 1.0),
            exponent: 25.0,
        }),
    );

    scene.add_shape(Box::new(sphere1));
    scene.add_shape(Box::new(sphere2));

    
    let light = PointLight::new(
        Point{
            x: 0.5,
            y: 0.5,
            z: 0.5,
        },
        0.8,
    );

    scene.add_light(Box::new(light));
    
    let scene = Arc::new(scene);

    let num_cores = match thread::available_parallelism() {
        Ok(num_cores) => num_cores.get(),
        Err(_) => 4
    };

    println!("Running {num_cores} tasks");

    let chunks = camera.get_pixels().chunks(num_cores);

    let timer = Instant::now();
    thread::scope(|scope|{
        for chunk in chunks {
            let camera = camera.clone();
            let scene = scene.clone();
            let integrator = integrator.clone();
            scope.spawn(move || {
                for pixel in chunk {
                    let (i, j) = pixel.position;

                    let radiance = (0..spp)
                        .into_iter()
                        .filter_map(|_| camera.sample_ray(i, j))
                        .map(|ray| integrator.sample_radiance(&ray, &scene))
                        .reduce(|accum, radiance| accum + radiance);

                    if let Some(radiance) = radiance {
                        let f = 1.0 / spp as f32;
                        *pixel.color.write().unwrap() = f*radiance;
                    }
                }
            });
        }
    });
    println!("Finished in {:.3}s", timer.elapsed().as_secs_f32());

    camera
        .get_sensor()
        .save("image.png")
        .expect("Error writing file");
}
