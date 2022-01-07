fn main() {
    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    // // rust does'nt allow to move one element out and change ownership to another
    // // variable, since it has to keep track of ownership of every element in vectar
    // // so either pop , or remove element swap last element in it's place or
    // // remove element from particular index and swap it with new element
    // let third = v[2];
    // let fifth = v[4];

    println!("v vector: {:?}", v);

    let fifth = v.pop().expect("vector empty!");
    println!("fifth: {}", fifth);

    let second = v.swap_remove(1);
    println!("second after removed and swapped: {}", second);
    println!("v vector afer: {:?}", v);

    let third = std::mem::replace(&mut v[2], "substitute".to_string());
    println!("third element: {}", third);

    println!("v vector: {:?}", &v);
}
