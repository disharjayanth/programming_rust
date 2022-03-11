#[derive(Debug)]
struct Onesuch;

fn main() {
    let o = Onesuch;
    println!("{:?}", o);

    // Range { start:3, end: 10 };
    // shorthand 3..10
    for i in 3..10 {
        println!("{}", i);
    }
}
