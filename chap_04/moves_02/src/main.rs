use std::vec;

fn main() {
    let s = vec!["udon".to_string(), "ramen".to_string(), "soba".to_string()];
    let t = s;
    // let u = s;
    // println!("s : {:?}", s);
    println!("t : {:?}", t);
    // println!("u : {:?}", u);

    // // if at all want to make a deep copy of vector
    // let a = s.clone();
    // let b = s.clone();
    // let c = s.clone();
}
