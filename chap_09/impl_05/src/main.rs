#[derive(Debug)]
pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    /// Push a character onto the back of a queue.
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    /// Pop a character off the front of a queue. Return 'Some(c)' if there
    /// was a character to pop, or 'None' if the queue was empty.
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Bring the elements in younger over to older, and put them in
            // the promised order.
            use std::mem::swap;
            swap(&mut self.younger, &mut self.older);
            self.older.reverse();
        }

        // Now older is guranteed to have something, Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }
}

fn main() {
    let mut queue = Queue {
        older: vec!['a', 'b', 'c'],
        younger: vec!['a', 'b', 'c', 'd'],
    };

    queue.push('e');

    println!("{:?}", queue);
}
