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

    // 'outerloop: for i in 1..5 {
    //     println!("*Loop: {i}");
    //     for j in 1..5 {
    //         if i == 3 {
    //             break 'outerloop;
    //         }
    //         println!("{j}");
    //     }
    // }

    for i in 1..10 {
        if i % 2 == 0 {
            continue;
        }
        println!("{i}");
    }

    let mut i = 0;
    let num = 'outerloop: loop {
        i += 1;
        println!("****Loop: {i}");
        for j in 1..10 {
            println!("{j}");
            if i == 7 {
                break 'outerloop i;
            }
        }
    };

    println!("Loop exited at: {num}");
}
