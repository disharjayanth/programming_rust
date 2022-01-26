fn main() {
    let some_strings = vec![
        "Hello".to_string(),
        "world".to_string(),
        "Hi".to_string(),
        "Hello".to_string(),
    ];

    for str in &some_strings {
        print!("{}\n", str);
    }
}
