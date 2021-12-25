fn main() {
    let a = true;
    let b = false;

    if a != b {
        println!("a != b");
    } else {
        println!("a = b");
    }

    assert_eq!(false as i32, 0);
    assert_eq!(true as i32, 1);

    let c = true as i32;
    println!("{}", c);

    let d = false as i32;
    println!("{}", d);
}
