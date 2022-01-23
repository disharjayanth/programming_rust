fn next_line<'a>() -> Option<&'a str> {
    Some("answer: ")
    // None
}

fn trim_comments_and_whitespaces(line: &mut String) -> &mut String {
    line.retain(|c| !c.is_whitespace());
    line.retain(|c| c != '\n');
    line.retain(|c| c != '\\');
    line
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

    let mut input_lines = vec![
        "Hello    world".to_string(),
        "Good luck\n".to_string(),
        " ".to_string(),
        "thanks a   lot".to_string(),
    ];

    for line in &mut input_lines {
        let trimmed = trim_comments_and_whitespaces(line);
        if trimmed.is_empty() {
            trimmed.push_str("IS EMPTY!!!");
            continue;
        }
    }

    println!("{:?}", input_lines);
}
