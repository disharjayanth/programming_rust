use std::ops::Range;
use std::time::Duration;

///Return true if two ranges overlap.
///
///    assert_eq!(ranges_13::overlap(0..7, 3..10), true);
///    assert_eq!(ranges_13::overlap(1..5, 101..105), false);
///
///If either range is empty, they don't count as overlapping.
///
///    assert_eq!(ranges_13::overlap(0..0, 0..10), false);
///
pub fn overlap(r1: Range<usize>, r2: Range<usize>) -> bool {
    r1.start < r1.end && r2.start < r2.end && r1.start < r2.end && r2.start < r1.end
}

struct Sunlight {}

///Let the sun shine in and run the simulation for a given
///amount of time.
///
///     # use fern_sim::Terrarium;
///     # use std::time::Duration;
///     # let mut tm = Terrarium::new();
///     tm.apply_sunlight(Duration::from_sec(60));
impl Sunlight {
    pub fn apply_sunlight(&mut self, time: Duration) {}

    ///Upload all local terrariums to the online gallery.
    ///
    ///```no_run
    ///let mut session = fern_sim::connect();
    /// session.upload_all();   
    pub fn upload_all(&mut self) {}
}
