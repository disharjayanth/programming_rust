#[derive(Debug)]
pub struct Bounds(usize, usize);

fn main() {
    let bounds = Bounds(2, 4);

    println!("{:?}", bounds);

    assert_eq!(bounds.0 * bounds.1, 8);
}
