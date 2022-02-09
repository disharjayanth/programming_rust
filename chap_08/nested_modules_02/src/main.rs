// mod plant_structures {
//     pub mod roots {}

//     pub mod stems {}

//     pub mod leaves {}
// }

mod plant_structures {
    pub mod roots {
        pub mod products {
            pub(in crate::plant_structures::roots) struct Cytokinin {}
        }

        use products::Cytokinin;
    }

    // use roots::products::Cytokinin;  // error because Cytokinin is private
}

// use plant_structures::roots::products::Cytokinin; // error because Cytokinin is private

fn main() {
    println!("Hello, world!");
}
