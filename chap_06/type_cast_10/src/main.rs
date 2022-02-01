fn main() {
    let x = 18;
    let index = x as u32;

    println!("{index}");

    let a = true;
    let b = a as i32;
    println!("{b}");

    let d: u8 = 200;
    let e = d as char;
    println!("{d} u8 in char is {e}");
}
