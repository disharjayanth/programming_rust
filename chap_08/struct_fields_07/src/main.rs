mod fern;

fn main() {
    let fern = fern::Fern {
        roots: fern::RootSet {},
        stems: fern::StemSet {},
    };

    println!("{:?}", fern);
}
