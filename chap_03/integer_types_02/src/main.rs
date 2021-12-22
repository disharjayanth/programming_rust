fn main() {
    // integer literals
    let a = 116i8;
    println!("a in form of 116 in 8 bit integer: {}", a);

    let b = 0xcafeu32;
    println!(
        "b in form of hexadecimal 0xcafe , in decimal form is: {}",
        b
    );

    let c = 0b0010_1010;
    println!(
        "c in form of binary 0b0010_1010 , in decimal form is: {}",
        c
    );

    let d = 0o106;
    println!("d in form of octal 0o106 , in decimal form is: {}", d);

    let e = b'\'';
    println!(
        "e is b'X' where X is any character to be encoded to ASCII stored as u8: {}",
        e
    );

    let d = b'\\';
    println!(
        "d is b'X' where X is backslash '\\' and in ASCII encoded(stored as u8) form is: {}",
        d
    );

    let e = b'\n';
    println!("new line \\n in ASCII form stored as u8: {}", e);

    let f = b'\r';
    println!("carriage return '\\r' in ASCII form: {}", f);

    let g = b'\t';
    println!("tab \\t in ASCII form: {}", g);

    // converting one integer type to another:
    assert_eq!(10_i8 as u16, 10u16);
    assert_eq!(2525_u16 as i16, 2525i16);

    assert_eq!(-1_i16 as i32, -1_i32);
    assert_eq!(65535_u16 as i32, 65535i32);

    assert_eq!(1000_i16 as u8, 232u8);
    assert_eq!(65535_u32 as i16, -1_i16);

    assert_eq!(-1_i8 as u8, 255_u8);
    assert_eq!(255_u8 as i8, -1_i8);

    assert_eq!(2_u16.pow(4), 16);
    assert_eq!((-4_i32).abs(), 4);
    assert_eq!(0b101101_u8.count_ones(), 4);
}
