use std::cmp::Ordering;
use std::mem::size_of;

#[derive(Debug)]
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

fn main() {
    println!("{:?}", compare(10, 2));

    assert_eq!(size_of::<Ordering>(), 1);

    assert_eq!(HttpStatus::Ok as i32, 200);
}
