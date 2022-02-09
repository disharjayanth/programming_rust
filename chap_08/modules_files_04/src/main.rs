mod spores;

mod plant_structures;

fn main() {
    let spores = spores::Spore {};
    let leaves = plant_structures::leaves::Leaves {
        name: "Mango Leaves".to_string(),
    };

    let xylem = plant_structures::stems::xylem::Xylem {
        name: "Xylem".to_string(),
    };

    let phloem = plant_structures::stems::phloem::Phloem {
        name: "phloem".to_string(),
    };

    println!("{:?}", spores);
    println!("{:?}", leaves);

    println!("{:?}", xylem);
    println!("{:?}", phloem);
}
