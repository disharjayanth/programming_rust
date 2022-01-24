use std::{
    fs::File,
    io::{Read, Write},
};

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

    let mut read = File::open("hello.go")?;
    let mut contents = String::new();
    read.read_to_string(&mut contents)?;
    println!("contents of hello.go file: {}", contents);
    Ok(())
}
