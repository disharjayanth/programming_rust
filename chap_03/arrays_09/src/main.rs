fn main() {
    let lazy_caterer = [1, 2, 3, 4, 7, 11, 16];
    println!("{:#?}", lazy_caterer);

    let taxonomy = ["Animalia", "Arthropoda", "Insecta"];
    println!("{:#?}", taxonomy);

    let long_array = [true; 10];
    println!("{:?}", long_array);

    let mut sieve = [true; 10000];
    for i in 2..100 {
        if sieve[i] {
            let mut j = i * i;
            while j < 10000 {
                sieve[j] = false;
                j += i;
            }
        }
    }

    assert!(sieve[211]);
    assert!(!sieve[9876]);

    println!("sieve[9876] : {}", sieve[9876]);

    let mut chaos = [3, 5, 4, 1, 2];
    chaos.sort();

    println!("Sorted chaos : {:?}", chaos);
}
