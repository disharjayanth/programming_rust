#[derive(Debug)]
pub struct RootSet {}

#[derive(Debug)]
pub struct StemSet {}

#[derive(Debug)]
pub struct Fern {
    pub roots: RootSet,
    pub stems: StemSet,
}
