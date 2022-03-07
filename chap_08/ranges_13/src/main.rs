use ranges_13;

fn main() {
    assert_eq!(ranges_13::overlap(0..7, 3..10), true);
    assert_eq!(ranges_13::overlap(1..5, 101..105), false);
}
