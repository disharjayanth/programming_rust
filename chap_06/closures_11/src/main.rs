fn main() {
    let is_even = |x| x % 2 == 0;
    println!(
        "is_even is a closure to check if given num is even: {}",
        is_even(2)
    );

    let is_odd = |x| x % 2 != 0;
    println!(
        "is_odd is a closure to check if given num is odd: {}",
        is_odd(1)
    );
}
