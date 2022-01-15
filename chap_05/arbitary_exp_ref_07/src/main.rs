fn factorial(n: usize) -> usize {
    return (1..n + 1).product();
}

fn main() {
    let r = &factorial(6);
    println!("r: {}", r);

    assert_eq!(*r, 720);

    assert_eq!(r + &1009, 1729);
}
