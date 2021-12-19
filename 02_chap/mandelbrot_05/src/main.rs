use num::Complex;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex{ re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn main() {
    let comp: Complex<f64> = Complex{re : 1.0, im: 2.0};
    println!("{:?}", escape_time(comp, 4))
}