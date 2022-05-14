#[derive(Debug)]
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }

        if slice[i] > *greatest {
            greatest = &slice[i];
        }
    }

    Extrema {
        greatest: greatest,
        least: least,
    }
}

fn main() {
    let e = Extrema {
        greatest: &10,
        least: &12,
    };

    println!("{:?}", e);

    let a = [0, -3, 0, 15, 40];
    let e = find_extrema(&a);
    assert_eq!(-3, *e.least);
    assert_eq!(40, *e.greatest);
}
