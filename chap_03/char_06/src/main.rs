fn main() {
    let a = '\'';
    println!("a : {}", a);

    let b = '\\';
    println!("b : {}", b);

    let c = '\n';
    println!("c : {}", c);

    let d = '\r';
    println!("d : {}", d);

    let e = '\t';
    println!("e : {}", e);

    let f = 'a';
    println!("f : {}", f);

    let g = '*';
    println!("g : {}", g);

    // // in hexadecimal
    let h = '\x2A';
    println!("h : {}", h);

    let i = '\x2B';
    println!("i : {}", i);

    let j = '\u{CA0}';
    println!("j : {}", j);

    let k = '\u{CA1}';
    println!("k : {}", k);

    let l = '\u{0CA6}';
    let k = '\u{0CC6}';
    println!("l , k : {}{}", l, k);

    let l = '*' as i32;
    println!("l : {}", l);

    let m = 'ಠ' as u16;
    println!("m : {}", m);

    assert_eq!('ಠ' as u16, 0xca0);
}