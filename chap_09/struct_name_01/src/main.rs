#[derive(Debug)]
// Both struct and it's fields are public.
pub struct GreyscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize),
}

// A rectangle of eight-bit grayscale pixels.
// GreyscaleMap struct is public but fields are private.
// pub struct GreyscaleMap {
//     pixels: Vec<u8>,
//     size: (usize, usize),
// }

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GreyscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GreyscaleMap { pixels, size }
}

fn main() {
    let gresyscalemap_1 = GreyscaleMap {
        pixels: vec![1, 2, 3, 4],
        size: (4, 8),
    };

    println!("{:?}", gresyscalemap_1);
    println!(
        "pixel: {:?}, size: {:?}",
        gresyscalemap_1.pixels, gresyscalemap_1.size
    );

    let width = 10;
    let height = 1;

    let image = GreyscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };

    println!("{:?}", image);
    println!("pixels: {:?}, size: {:?}", image.pixels, image.size);

    let some_image = new_map((2, 1), vec![2, 2]);
    println!("{:?}", some_image);

    assert_eq!(some_image.size, (2, 1));
    assert_eq!(some_image.pixels.len(), 2);
}
