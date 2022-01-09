use std::rc::Rc;

fn main() {
    let s = Rc::new("shirataki".to_string());
    let t = s.clone();
    let u = s.clone();
    println!("{:?}", s);

    assert!(s.contains("shira"));
    assert_eq!(t.find("taki"), Some(5));
    println!("{} are quite chewy, almost bouncy, but lack flavor", u);

    // // u is not mutable
    // u.push_str(" noodles");
}
