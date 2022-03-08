use image::{GenericImage, Rgb, RgbImage};

fn main() {
    let img = image::open("/Users/disharjayantha/Documents/10-11.jpg").unwrap();

    println!("dimensions: {:?}", img.dimensions());
    println!("{:?}", img.color());

    let mut img = RgbImage::new(32, 32);

    for x in 15..=17 {
        for y in 8..24 {
            img.put_pixel(x, y, Rgb([255, 0, 0]));
            img.put_pixel(y, x, Rgb([255, 0, 0]));
        }
    }

    img.save("test.png").unwrap();
}
