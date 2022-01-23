use std::vec;

fn example() -> Option<i32> {
    Some(1)
    // None
}

fn main() {
    // Moving indexed value of Copy and Move type
    let a = vec![1, 2, 3, 4];
    let first = a[0];
    let second = a[1];

    println!("first and second: {first}, {second}");

    let aa = vec!["Hello".to_string(), "world!".to_string()];
    let first_1 = &aa[0];
    let second_2 = &aa[1];

    println!("{first_1} {second_2}");

    if let Some(id) = example() {
        println!("There is some value in id: {}", id);
    } else {
        println!("None");
    }

    let mut i = 0;

    // while loop
    while i < 5 {
        i+=1;
        println!("i: {}", i);
    }

    // while let loop
    // while let Some(i) = example() {
    //     println!("i: {}", i);
    // }

    // loop
    println!("loop");
    loop {
        i += 1;
        println!("i: {i}");
        if i == 10 {
            break;
        }
    }

    // for in loop
    let mut some_string = vec!["hello".to_string(), "hi".to_string(), "ayyy".to_string(), "alo".to_string()];

    // below for loop moves each element in some_string to str and finally 
    // all elements in some_string are moved and it's empty 
    // if move is not required then use reference to iterator ex. &Vec<String>
    // for str in some_string {
    //     println!("{}", str);
    // }

    // below code throws error since it is accessing element of moved variable
    // some_string[0];

    for str in &some_string {
        println!("{}", str)
    }

    for str in &mut some_string {
        str.push('\n')
    }

    println!("{:?}", some_string);

    // printing again to see effect after appening '\n'
    // to each word using print!
    for str in &some_string {
        print!("{}", str)
    }
}
