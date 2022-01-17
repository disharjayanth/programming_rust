fn smallest(v: &[i32]) -> &i32 {
    let mut smallest = &v[0];
    for r in &v[1..] {
        if *r < *smallest {
            smallest = r;
        }
    }
    smallest
}

fn main() {
    let s;
    {
        let parabola = [9, 4, 1, 0, 1, 4, 9];
        s = smallest(&parabola);
        assert_eq!(*s, 0);
        println!(
            "Smallest value in parabola array: {:?} is: {}.",
            parabola, s
        );
    }
}
