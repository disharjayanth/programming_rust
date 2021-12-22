fn main() {
    // below code overflows
    // let mut i = 1;
    // loop {
    //     println!("loop");
    //     i *= 10;
    // }

    // below code overflows too
    // let mut i: i32 = 1;
    // loop {
    //     // panic: multiplication overflowed (in any build)
    //     i = i.checked_mul(10).expect("multiplication overflowed")
    // }

    // sum of 10 and 20 can be represented as u8
    assert_eq!(10_u8.checked_add(20), Some(30));

    // Unfortunately, the sum of 100 ans 200 cannot
    assert_eq!(100_u8.checked_add(200), None);

    assert_eq!(100_u8.checked_add(250), None);

    let x: u8 = 1;
    let y: u8 = 4;
    let sum = x.checked_add(y).unwrap();
    println!("sum: {}", sum);

    // signed divison can overflow too
    // signed n-bit type can represent -2n-1, but not 2n-1 (to power n)
    assert_eq!((-128_i8).checked_div(-1), None);
}
