struct Person{
    name: String ,
    age: u16 ,
    is_admin: bool ,
    country: String 
}

impl Person {
    fn about(&self) -> String {
        return "The name of person is ".to_string() + &self.name.to_string() + &" of age ".to_string() + &self.age.to_string() + &" lives in ".to_string() + &self.country.to_string() + &" and is admin ".to_string() + &self.is_admin.to_string();
    }
}

fn main() {
    let person = Person{
        name: "John Doe".to_string() ,
        age: 28 ,
        is_admin: true ,
        country: "US".to_string()
    };

    let msg = {
        person.about()
    };

    println!("msg: {}", msg);
}
