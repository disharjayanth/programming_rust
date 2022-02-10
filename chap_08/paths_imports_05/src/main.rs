use std::collections::{HashMap, HashSet};
use std::mem;

fn main() {
    let mut s1 = 20;
    let mut s2 = 10;

    println!("Before swap: {s1} {s2}");

    if s1 > s2 {
        mem::swap(&mut s1, &mut s2);
    }

    println!("After swap: {s1} {s2}");

    let mut bookreviews = HashMap::new();

    bookreviews.insert(
        "Adventures of Harry Potter".to_string(),
        "My favorite book".to_string(),
    );

    bookreviews.insert("Grim's fairy tail".to_string(), "Master piece".to_string());

    let to_find = [
        "Adventures of Harry Potter",
        "some random book",
        "Grim's fairy tail",
    ];

    for &find in &to_find {
        match bookreviews.get(find) {
            Some(book) => {
                println!("found review: {}", book);
            }
            None => {
                println!("No reviews")
            }
        }
    }

    let mut books = HashSet::new();

    books.insert("A dance with dragons");
    books.insert("Mocking bird");
    books.insert("The odyssey");
    books.insert("The great gatsby");

    println!("****Books in HashSet****");
    for &book in &books {
        println!("{book}");
    }
}
