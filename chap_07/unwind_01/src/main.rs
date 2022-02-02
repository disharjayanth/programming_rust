fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

fn main() {
    // divide by 0 error
    // println!("{}", pirate_share(10, 0));
    println!("{}", pirate_share(10, 2));
}
