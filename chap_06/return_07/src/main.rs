use std::{fs::File, io::Write};

fn some_string<'a>() -> &'a str {
    "Hello world!"
}

fn some_fn() {
    return;
}

fn main() -> std::io::Result<()> {
    println!("{}", some_string());
    println!("{:?}", some_fn());

    let mut output = File::create("hello.go")?;
    output.write(
        b"package main \n \nimport \"fmt\" \n \nfunc main() { \n \tfmt.Println(\"Hello world\") \n}",
    )?;
    Ok(())
}
