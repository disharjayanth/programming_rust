fn main() {
    let a = -1.5625;
    println!("a : {}", a);

    let b = 2.;
    println!("b : {}", b);

    let c = 0.25;
    println!("c : {}", c);

    let d = 1e4;
    println!("d : {}", d);

    let e = 40f32;
    println!("e : {}", e);

    let f = 9.109_383_56e-31f64;
    println!("f : {}", f);

    assert!((-1. / f32::INFINITY).is_sign_negative());

    assert_eq!(-f32::MIN, f32::MAX);

    let g = 2f64.sqrt();
    println!("sqrt of 2 : {}", g);

    assert_eq!((-1.01f64).floor(), -2.0);

    println!("sqrt of 2 using suffix f64 type: {}", (2.0_f64).sqrt());
    println!("sqrt of 2 using function f64 sqrt: {}", f64::sqrt(2.0));
}
