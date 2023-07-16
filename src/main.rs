use walnut::{
    Sensor,
    Color,
};

fn main() {

    let mut image = Box::new(Sensor::constant(Color::new(0.0, 1.0, 0.0), 800, 600));

    for i in 0..image.width() {
        let pixel = image.get_mut(i, 200).unwrap();
        *pixel = Color::new(1.0, 0.0, 0.0);
    }

    image.save("image.png").unwrap();
}