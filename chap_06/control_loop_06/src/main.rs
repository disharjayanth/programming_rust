fn next_line<'a>() -> Option<&'a str> {
    Some("answer: ")
    // None
}

fn main() {
    let answer = loop {
        if let Some(line) = next_line() {
            if line.starts_with("answer: ") {
                break line;
            }
        } else {
            break "answer: nothing";
        }
    };

    println!("{answer}");
}
