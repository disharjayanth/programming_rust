fn g<'a>(p: &'a i32) {}

fn main() {
    let x = 10;
    g(&x);
}
