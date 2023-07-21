use walnut::*;

use std::thread;
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let spp = 255;
    let sensor = Sensor::zero(800, 800);
    let camera = Arc::new(PinholeCamera::new(sensor, 45.0));

    let sphere = Arc::new(Sphere {
        center: Point {
            x: 0.0,
            y: 0.0,
            z: -5.0,
        },
        radius: 1.0,
    });

    let num_cores = match std::thread::available_parallelism() {
        Ok(num_cores) => num_cores.get(),
        Err(_) => 4
    };

    println!("Running {num_cores} tasks");

    let chunks = camera.get_pixels().chunks(num_cores);

    let timer = Instant::now();
    thread::scope(|scope|{
        for chunk in chunks {
            let camera = camera.clone();
            let sphere = sphere.clone();
            scope.spawn(move || {
                for pixel in chunk {
                    let (i, j) = pixel.position;
    
                    let n = (0..spp)
                        .into_iter()
                        .filter_map(|_| camera.sample_ray(i, j))
                        .filter_map(|ray| sphere.intersect(&ray))
                        .map(|si| si.normal)
                        .reduce(|accum, normal| accum + normal);
    
                    if let Some(n) = n {
                        let f = 1.0 / spp as f32;
                        *pixel.color.write().unwrap() = Color::new(f*n.x, f*n.y, f*n.z);
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
