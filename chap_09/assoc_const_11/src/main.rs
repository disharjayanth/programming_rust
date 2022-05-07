#[derive(Debug)]
pub struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    const ZERO: Vector = Vector { x: 0.0, y: 0.0 };
    const UNIT: Vector = Vector { x: 1.0, y: 1.0 };
}

impl Vector {
    const NAME: &'static str = "Vector";
    const ID: u32 = 18;
}

fn main() {
    let zero = Vector::ZERO;
    println!("{:?}", zero);

    let scaled = Vector::UNIT;
    println!("{:?}", scaled);

    let name = Vector::NAME;
    println!("{:?}", name);

    let id = Vector::ID;
    println!("{:?}", id);
}
