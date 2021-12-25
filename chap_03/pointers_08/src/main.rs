fn main() {
    // mutable and immutable pointers
    let a = String::from("Hello world!");
    println!("a String : {}", a);

    let b = &a;
    println!("b is pointer to a : {}", b);

    let c = String::from("bye!");

    let mut d = &c;

    println!("d : {}", d);

    d = &a;

    println!("d : {}", d);

    // Boxes (to allocate memory on heap)
    // simples way to allocate to heap
    let aa = (12, "carrot");
    let bb = Box::new(aa);
    println!("bb : {:?}", bb);

    // raw pointers are not safe
}
