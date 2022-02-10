use super::roots::Roots;

#[derive(Debug)]
pub struct Leaves {
    pub name: String,
}

pub fn some_root_fn() -> Roots {
    let root = Roots {
        name: "some root".to_string(),
    };

    root
}
