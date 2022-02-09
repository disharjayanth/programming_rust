mod plant_structures;

mod spores;

fn main() {
    let leaves = plant_structures::leaves::Leaves {
        name: "Mango leaves".to_string(),
    };
    let roots = plant_structures::roots::Roots {
        name: "Mango roots".to_string(),
    };
    let stems = plant_structures::stems::Stems {
        name: "Mango Stem".to_string(),
    };

    println!("{:?} {:?} {:?}", leaves, roots, stems);

    let some_spore = spores::Spore {};
    println!("{:?}", some_spore);
}
