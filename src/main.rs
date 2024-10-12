use walnut::*;

use std::sync::Arc;
use std::thread;
use std::time::Instant;

fn main() {
    let spp = 256;
    let sensor = Sensor::zero(800, 800);
    let camera = Arc::new(PinholeCamera::new(sensor, 75.0));
    let integrator = Arc::new(PathIntegrator::new(4, 2));

    let mut scene = Scene::new();

    let sphere1 = Sphere::new(
        Point {
            x: 0.0,
            y: 0.0,
            z: -2.5,
        },
        1.0,
        Box::new(PhongMaterial {
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
        Box::new(PhongMaterial {
            albedo: Color::new(0.0, 0.2, 0.8),
            specular: Color::new(1.0, 1.0, 1.0),
            exponent: 25.0,
        }),
    );

    let ground = InfinitePlane::new(
        Point {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Box::new(DiffuseMaterial {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    );

    let back = InfinitePlane::new(
        Point {
            x: 0.0,
            y: 0.0,
            z: -4.0,
        },
        Vector {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Box::new(DiffuseMaterial {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    );

    let top = InfinitePlane::new(
        Point {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        },
        Vector {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        Box::new(DiffuseMaterial {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    );

    let left = InfinitePlane::new(
        Point {
            x: -4.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        },
        Box::new(DiffuseMaterial {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    );

    let right = InfinitePlane::new(
        Point {
            x: 4.0,
            y: 0.0,
            z: 0.0,
        },
        Vector {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        },
        Box::new(DiffuseMaterial {
            albedo: Color::new(0.5, 0.5, 0.5),
        }),
    );

    scene.add_shape(Box::new(sphere1));
    scene.add_shape(Box::new(sphere2));
    scene.add_shape(Box::new(ground));
    scene.add_shape(Box::new(back));
    scene.add_shape(Box::new(top));
    scene.add_shape(Box::new(left));
    scene.add_shape(Box::new(right));

    let light = PointLight::new(
        Point {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        0.8,
    );

    scene.add_light(Box::new(light));

    let scene = Arc::new(scene);

    let num_cores = match thread::available_parallelism() {
        Ok(num_cores) => num_cores.get(),
        Err(_) => 4,
    };

    println!("Running {num_cores} tasks");

    let chunks = camera.get_pixels().chunks(num_cores);

    let timer = Instant::now();
    thread::scope(|scope| {
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
                        *pixel.color.write().unwrap() = f * radiance;
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
