mod Person {
    pub struct Person {
        name: String,
    }

    pub fn print_person() {}

    pub fn edit_person(person: &mut Person) -> Person {
        Person {
            name: "edited name".to_string(),
        }
    }
}

fn main() {}
