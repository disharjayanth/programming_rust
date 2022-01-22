use std::collections::HashMap;

struct Params<'a> {
    names: HashMap<&'a str, String>
}

fn main() {
    let b = 10;
    let a = 2;

    if a > b {
        println!("a is greater than b....")
    } else if b > a {
        println!("b is greater than a....")
    } else {
        println!("a is equal to b....")
    }

    let code= 8;

    match code {
        0 => {println!("OK") }
        1 => println!("Wires Tangled") ,
        2 => println!("User Asleep") ,
        _ => println!("Unrecognized Error {}", code)
    }

    let mut params = Params {
        names: HashMap::new()
    };

    params.names.insert("John", "Doe".to_string());

    match params.names.get("John") {
        Some(name) => println!("Hello, {}", name),
        None => println!("Greetings!, stranger.")
    }

    let is_hockey_season = false;
    
    let best_sports_team = if is_hockey_season {
        "Predators"
    } else {
        "loosers"
    };

    println!("{}", best_sports_team);
}
