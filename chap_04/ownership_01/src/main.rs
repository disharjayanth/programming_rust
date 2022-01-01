fn print_padavan() {
    let mut padovan = vec![1, 1, 1];
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("padovan[1..10]: {:?}", padovan);
    println!(
        "length and capacity: {} {}",
        padovan.len(),
        padovan.capacity()
    );
}

struct Person {
    name: String,
    birth: i32,
}

fn main() {
    print_padavan();
    println!("******");

    let mut composers = Vec::new();

    println!(
        "Length and capacity of vec: {} {}",
        composers.len(),
        composers.capacity()
    );

    composers.push(Person {
        name: "Palestrina".to_string(),
        birth: 1525,
    });
    composers.push(Person {
        name: "Dowland".to_string(),
        birth: 1563,
    });
    composers.push(Person {
        name: "Lully".to_string(),
        birth: 1632,
    });

    for composer in &composers {
        println!("{}, born {}", composer.name, composer.birth);
    }

    println!(
        "Length and capacity of vec: {} {}",
        composers.len(),
        composers.capacity()
    );
}
