fn main() {
    let a = ("Brazil", 1985);
    println!("{:#?}", a);
    println!("{} , {}",a.0, a.1);

    let text = "Hello, there! How are you?";
    let (head, tail) = text.split_at(21);
    println!("head: {} \ntail: {}", head, tail);

    let text1 = "Hi, Im good, How are you?";
    let temp = text1.split_at(21);
    let head = temp.0;
    let tail = temp.1;
    println!("head : {} \ntail : {}", head, tail);

    // example with swapping using mutable
    let mut text2 = "Hello";
    let mut text4 = "world";
    let b = &mut text2;
    let c = &mut text4;
    std::mem::swap(b, c);
    println!("{} {}", b, c);
}
