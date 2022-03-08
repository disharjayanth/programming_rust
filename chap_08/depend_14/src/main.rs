use image::GenericImage;

fn main() {
    let img = image::open("/Users/disharjayantha/Documents/10-11.jpg").unwrap();

    println!("dimensions: {:?}", img.dimensions());
    println!("{:?}", img.color());
}
