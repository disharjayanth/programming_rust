fn main() {
    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    println!("{}", oodles);

    let poodles = "ಠ_ಠ";
    assert_eq!(poodles.len(), 7);

    let poodles_in_bytes = br####"A"####;
    println!("poodles in bytes: {:?}", poodles_in_bytes);

    let mut s: &str = "hello";
    println!("s: {}", s);
    // cannot modify s , throws error
    // s[0] = 'c';
    // s.push('\n');

    s = "hi";
    println!("{}", s);

    let error_message = "too many pets".to_owned();
    // // same as above
    // let error_message = "too many pets".to_string();
    println!("error message: {}", error_message);

    assert!("ONE".to_lowercase() == "one");
    assert_eq!("ONE".to_lowercase(), "one");

    assert!("peanut".contains("nut"));

    assert_eq!("ಠ_ಠ".replace("ಠ", "■"), "■_■");
    println!("{}", "ಠ_ಠ".replace("ಠ", "■"));

    assert_eq!("          clean\n".trim(), "clean");

    let words = "veni, vidi, vici".split(", ");

    // for word in words {
    //     println!("{}", word);
    // }

    for word in words {
        assert!(word.starts_with("v"));
    }
}
