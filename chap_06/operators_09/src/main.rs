fn main() {
    let a = -100;
    println!("{a}");

    // error: can't apply unary `-` to type u32
    // println!("{}", -100u32);

    // error: unexpected + unary operator
    // println!("{}", +100);

    let b = 1234.567 % 10.0;
    println!("{b}");

    let hi = 0xe0;
    println!("{hi}");

    println!("{}", 10 << 1);
    println!("{}", 10 >> 1);
}
