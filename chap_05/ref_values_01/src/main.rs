use std::{collections::HashMap, vec};

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!(" {}", work);
        }
    }
}

fn main() {
    let mut tables = Table::new();

    tables.insert(
        "Gesualdo".to_string(),
        vec![
            "many mandrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );

    tables.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The calling of st. matthew".to_string(),
        ],
    );

    tables.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    show(tables);
}
