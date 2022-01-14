use std::vec;

fn main() {
    // reference
    let x = 10;
    let r = &x;
    println!("{}", r);
    assert!(*r == 10);

    // mutable reference
    let mut y = 32;
    let m = &mut y;
    *m += 32;
    println!("m: {}", *m);
    assert!(*m == 64);
    println!("y: {}", y);

    let p = &mut y;
    *p += 32;
    println!("p: {}", p);

    struct Person {
        name: &'static str,
        bechdel_pass: bool,
    }

    let p1: Person = Person {
        name: "Aria: The animation",
        bechdel_pass: true,
    };

    let p1_ref = &p1;

    println!("accessing p1_ref(reference to p1) name: {}", p1_ref.name);
    println!(
        "accessing p1_ref(reference to p1) bechdel_pass: {}",
        p1_ref.bechdel_pass
    );

    let mut v = vec![1973, 1968];
    println!("before sort v: {:?}", v);
    v.sort();
    // . adds &mut to left operand
    // . either borrows a mutable reference (&mut *some_variable*) or
    // deferences (*v.) = v.
    // (&mut v).sort();
    println!("after sort v: {:?}", v);
}
