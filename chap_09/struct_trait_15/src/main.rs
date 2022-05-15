#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.0, y: 2.0 };

    println!("p: {:?}", p);

    let q = p;

    println!("p: {:?}", p);
    println!("q: {:?}", q);

    if p.x > p.y {
        println!("x is greater than y");
    } else {
        println!("y is greater than x");
    }
}
