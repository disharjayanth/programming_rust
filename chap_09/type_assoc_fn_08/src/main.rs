#[derive(Debug)]
struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }
}

fn main() {
    let mut q = Queue::new();
    println!("{:?}", q);
}
