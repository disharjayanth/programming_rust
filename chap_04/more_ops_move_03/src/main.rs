#[derive(Debug)]
struct Person {
    name: String,
    birth: i32,
}

fn main() {
    let mut s = "Govinda".to_string();
    println!("s value before reintialising: {}", s);
    s = "Siddhartha".to_string();
    println!("s value after reinitialising: {}", s);

    let mut a = "Govinda".to_string();
    let b = a;
    a = "Siddhartha".to_string();
    println!("a : {}, b : {}", a, b);

    let mut composers = Vec::new();
    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });

    println!(
        "composers value : {} {}",
        composers[0].name, composers[0].birth
    );
}
