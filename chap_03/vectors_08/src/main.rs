use std::vec;

fn main() {
    let mut primes = vec![2, 3, 5, 7];
    let product: i32 = primes.iter().product();
    println!("product: {}", product);

    primes.push(11);
    primes.push(13);
    println!("product after pushing more elements: {:?}", primes);

    assert_eq!(primes.iter().product::<i32>(), 30030);

    let mut pal : Vec<&str> = Vec::new();
    pal.push("step");
    pal.push("on");
    pal.push("no");
    pal.push("pets");

    assert_eq!(pal, vec!["step", "on", "no", "pets"]);

    println!("pets : {:?}", pal);

    let v : Vec<i32> = (0..5).collect();
    println!("v vector : {:?}", v);

    // A palindrome
    let mut palindrome = vec!["a man", "a plan", "a canal", "panama"];
    println!("value of palindrome variable: {:?}", palindrome);
    palindrome.reverse();
    println!("value of palindrome after reversing: {:?}", palindrome);

    let mut q : Vec<i32> = Vec::with_capacity(2);
    println!("Length of q : {}", q.len());
    println!("Capacity of q : {}", q.capacity());

    q.push(1);
    q.push(2);

    println!("Length of q after pushing elements: {}", q.len());
    println!("Capacity of q after pushing elements: {}", q.capacity());

    q.push(3);

    println!("Length of q after pushing elements: {}", q.len());
    println!("Capacity of q after pushing elements: {}", q.capacity());

    let mut v = vec![10, 20, 30, 40, 50];
    println!("v : {:?}", v);

    v.insert(3, 35);
    println!("v after inserting 35 at index 3 : {:?}", v);

    v.remove(2);
    println!("v after removing element at index 3: {:?}", v);

    let mut w = vec!["Some Puff", "Glass Gem"];
    println!("w : {:?}", w);
    println!("Popping 1 element : {:?}", w.pop());
    println!("Popping 1 element : {:?}", w.pop());
    println!("Popping 1 element : {:?}", w.pop());

    let languages : Vec<String> = std::env::args().skip(1).collect();

    for l in languages {
        if l.len() % 2 == 0 {
            println!("{} : {}", l, "functional");
        } else {
            println!("{} : {}", l, "imperative");
        }
    }
}
