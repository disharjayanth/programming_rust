use rand::prelude::*;

fn main() {
    if rand::random() {
        println!("char {}", rand::random::<char>());
    }

    let mut rng = rand::thread_rng();
    let y: f32 = rng.gen();

    println!("0 to 1: {}", y);

    let mut nums: Vec<i32> = (1..100).collect();
    nums.shuffle(&mut rng);

    println!("{:?}", nums);
}
