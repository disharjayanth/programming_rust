fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}

fn main() {
    println!("{}", pirate_share(10, 0));
}
