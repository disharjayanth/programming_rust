pub struct Sporangium {}

pub struct Gene {}

struct Cell {}

#[derive(Debug)]
pub struct Spore {}

pub fn produce_spore(factory: &mut Sporangium) -> Spore {
    Spore {}
}

pub(crate) fn genes(spore: &Spore) -> Vec<Gene> {
    let genes = Vec::<Gene>::new();
    genes
}

fn recombine(parent: &mut Cell) {}
