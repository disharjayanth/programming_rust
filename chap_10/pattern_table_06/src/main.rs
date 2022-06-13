fn match_int(n: i32) {
    match n {
        100 => println!("{} is n", n),
        200 => println!("{} is n", n),
        _ => println!("{} is n", n),
    }
}

fn match_range(n: i32) {
    match n {
        0..=100 => println!("n is in range from 0 to 100: {}", n),
        _ => println!("n is not in range: {}", n),
    }
}

fn match_var(n: String) {
    match n {
        name => println!("name is: {}", name),
    }
}

fn match_ref(n: i32) {
    match n {
        ref field => println!("n: {}", field),
    }
}

fn bind_subpattern(n: i32) {
    match n {
        val @ 0..=99 => println!("val is: {}", val),
        val @ 100..=200 => println!("val is more than 100: {}", val),
        _ => println!("val is {}", n),
    }
}

fn main() {
    match_int(100);
    match_int(1001);

    match_range(10);

    match_var("John".to_string());
    match_var("Smith".to_string());

    match_ref(12);

    bind_subpattern(98);
    bind_subpattern(102);
    bind_subpattern(200);
    bind_subpattern(1000);
}
