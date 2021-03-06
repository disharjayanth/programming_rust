pub mod leaves;
pub mod roots;
pub mod stems;

pub use self::leaves::Leaf;
pub use self::roots::Root;

use self::roots::RootSet;
use self::stems::StemSet;

pub enum FernType {
    Fiddlehead,
}

pub struct Fern {
    pub roots: RootSet,
    pub stems: StemSet,
}

impl Fern {
    pub fn new(_type: FernType) -> Fern {
        Fern {
            roots: vec![],
            stems: vec![stems::Stem { furled: true }],
        }
    }

    pub fn is_furled(&self) -> bool {
        !self.is_fully_unfurled()
    }

    pub fn is_fully_unfurled(&self) -> bool {
        self.stems.iter().all(|s| !s.furled)
    }
}

/// Create and return a [`VascularPath`] which represents the path of
/// nutrients from the given [`Root`][r] to the given [`Leaf`](leaves::Leaf).
///
/// [r]: roots::Root
pub fn trace_path(leaf: &leaves::Leaf, root: &roots::Root) -> VascularPath {
    VascularPath {
        from: leaf.x,
        to: root.x,
    }
}

#[doc(alias = "route")]
pub struct VascularPath {
    pub from: bool,
    pub to: bool,
}
