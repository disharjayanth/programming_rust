fn main() {
    let string1 = "somnambulance".to_string();
    let string2 = string1;

    let num1 = 36;
    let num2 = num1;

    // // cannot read string1 since it's uninitialised because ownership
    // // has changed from string1 to string2
    // println!("string1: {}", string1);
    println!("string2: {}", string2);

    // // where in num1 and num2 i32 value is copied around
    println!("num1: {}", num1);
    println!("num2: {}", num2);

    println!("*******");

    #[derive(Copy, Clone)]
    struct Label {
        number: u32,
    }

    fn print(l: Label) {
        println!("STAMP: {}", l.number);
    }

    let l = Label { number: 3 };

    print(l);

    // // cannot borrow a value which is moved. since Label struct
    // // does't implement Copy trait
    println!("My label number is : {}", l.number);
}
