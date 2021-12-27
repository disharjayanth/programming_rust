fn print(n: &[f64]) {
    println!("********");
    for elt in n {
        println!("{}", elt);
    }
    println!("--------");
}

fn main() {
    let v = vec![0.0, 0.707, 1.0, 0.707];
    let a = [0.0, -0.707, -1.0, -0.707];

    let sv = &v;
    let sa = &a;

    println!("value of v: {:?}", v);
    println!("value of a: {:?}", a);

    println!("slice of v sv: {:?}", sv);
    println!("slice of a sa: {:?}", sa);

    print(&a);

    print(&v);

    print(&v[0..3]);

    print(&a[1..3]);
}
