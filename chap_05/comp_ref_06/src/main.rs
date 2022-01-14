fn main() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    println!("rrx: {} and rry: {}", rrx, rry);
    assert!(rrx <= rry);
    assert!(rrx == rry);

    assert!(!std::ptr::eq(rx, ry));

    // type mismatch: &i32 vs &&i32
    // assert!(rx == rrx);

    assert!(rx == *rrx);
}
