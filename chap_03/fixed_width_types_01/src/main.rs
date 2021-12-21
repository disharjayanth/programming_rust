fn main() {
    let au: u8 = 10;
    let ai: i8 = -10;
    println!(
        "au is of type unsigned 8 bit integer: {} and ai is of type signed 8 bit integer: {}",
        au, ai
    );

    let bu: u16 = 100;
    let bi: i16 = -100;
    println!("bu u16: {} bi i16: {}", bu, bi);

    let cu: u32 = 10000;
    let ci: i32 = -100000;
    println!("cu u32: {}, ci i32: {}", cu, ci);

    let du: u64 = 2000000000200000000;
    let di: i64 = -2000000000200000000;
    println!("du u64: {}, di i64: {}", du, di);

    let eu: u128 = 322832032023202332283203202320233228320;
    let ei: i128 = -32283203202320233228320320232023322832;
    println!("eu u128: {}, ei i128: {}", eu, ei);

    let fu: usize = 3228320320238888888;
    let fi: isize = -3228320320238888888;
    println!("fu usize: {}, fi isize: {}", fu, fi);
}
