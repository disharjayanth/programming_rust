fn main() {
    // // below code overflows
    // let mut i = 1;
    // loop {
    //     println!("loop");
    //     i *= 10;
    // }

    // // below code overflows too
    // let mut i: i32 = 1;
    // loop {
    //     // panic: multiplication overflowed (in any build)
    //     i = i.checked_mul(10).expect("multiplication overflowed")
    // }

    // // sum of 10 and 20 can be represented as u8
    // assert_eq!(10_u8.checked_add(20), Some(30));

    // // Unfortunately, the sum of 100 ans 200 cannot
    // assert_eq!(100_u8.checked_add(200), None);

    // assert_eq!(100_u8.checked_add(250), None);

    // let x: u8 = 1;
    // let y: u8 = 4;
    // let sum = x.checked_add(y).unwrap();
    // println!("sum: {}", sum);

    // // signed divison can overflow too
    // // signed n-bit type can represent -2n-1, but not 2n-1 (to power n)
    // assert_eq!((-128_i8).checked_div(-1), None);

    // // wrapped operations return value equivalent to the mathematically
    // // correct result modulo the range of the value.
    // assert_eq!(100_u16.wrapping_mul(200), 20000);
    // assert_eq!(500_u16.wrapping_mul(500), 53392);

    // assert_eq!(500_i16.wrapping_mul(500), -12144);

    // assert_eq!(5_i16.wrapping_shl(17), 10);

    // // saturating operations return representable value that is closest to
    // // the mathematically correct result. result is clamed down to min/max
    // // value that type can represent
    // assert_eq!(32760_i16.saturating_add(10), 32767);
    // assert_eq!((-32760_i16).saturating_sub(10), -32768);

    // // overflowing operations return tuple(result, overflowed)
    // // result is wrapping version of function would return and
    // // overflowed is bool, whether an overflow occurred
    // assert_eq!(255_u8.overflowing_sub(2), (253, false));
    // assert_eq!(255_u8.overflowing_add(2), (1, true));

    // // few more examples on checked, wrapped, saturating, overflow
    // assert_eq!(100_i8.checked_add(27), Some(127));
    // assert_eq!(10_u8.checked_sub(11), None);
    // assert_eq!(128_u8.saturating_mul(3), 255);
    // assert_eq!(64_u16.wrapping_div(8), 8);
    // assert_eq!((-32768_i16).wrapping_rem(-1), 0);
    // assert_eq!((-128_i8).checked_neg(), None);
    // assert_eq!((-32768_i16).wrapping_abs(), -32768);
    // assert_eq!(3_u8.checked_pow(4), Some(81));
    // assert_eq!(10_u32.wrapping_shl(34), 40);
    // assert_eq!(40_u64.wrapping_shr(66), 10);
}
