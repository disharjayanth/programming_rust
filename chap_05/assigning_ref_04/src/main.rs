fn main() {
    let b = true;
    let x = 10;
    let y = 20;
    // mutable variable r where r currently pointer to x
    let mut r = &x;

    // if b is true r now points reference to y
    if b {
        r = &y;
    }

    assert!(*r == 20);
    println!("r: {}", r);
}
