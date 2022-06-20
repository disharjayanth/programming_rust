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

fn enum_pattern(n: Option<String>) {
    match n {
        Some(val) => println!("value is some: {}", val),
        None => println!("value is none."),
    }
}

enum TupleEnum {
    Name(i32, String),
    RGB(i32, i32, i32),
}

fn tuple_pattern(n: TupleEnum) {
    match n {
        TupleEnum::Name(key, value) => println!("name and key is: {} {}", key, value),
        TupleEnum::RGB(r, g, b) => println!("r {} g {} b {}", r, g, b),
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

    enum_pattern(Some("Hello world!".to_string()));
    enum_pattern(None);

    tuple_pattern(TupleEnum::Name(1, "John".to_string()));
    tuple_pattern(TupleEnum::RGB(21, 10, 44));
}
