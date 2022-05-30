use std::collections::HashMap;

#[derive(Debug)]
enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

fn main() {
    let mut some_name = HashMap::new();
    some_name.insert("Tokyo".to_string(), Json::Number(1.2));

    let place = Json::Object(Box::new(some_name));
    println!("{:?}", place);

    let humid = Json::Boolean(true);
    println!("{:?}", humid);

    let item = Json::Null;
    println!("{:?}", item);

    let name = Json::String("Jam".to_string());
    println!("{:?}", name);

    let places = Json::Array(vec![Json::Boolean(false), Json::Boolean(true)]);
    println!("{:?}", places);
}
