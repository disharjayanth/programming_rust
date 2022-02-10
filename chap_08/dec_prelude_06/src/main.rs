mod plant_structures;

fn main() {
    let leaves = plant_structures::leaves::Leaves {
        name: "some leaves".to_string(),
    };

    println!("{:?}", leaves);

    println!("{:?}", plant_structures::leaves::some_root_fn());
}
