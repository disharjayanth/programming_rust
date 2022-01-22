struct User {
    name: String 
}

impl User {
    fn nickname(self) -> String {
        self.name
    }

    fn has_nickname(&self) -> bool {
        true
    }

    fn register(&mut self, name: &String) {
        self.name = name.to_string();
    }
}

fn main() {
    let a = 10;
    println!("a: {}", a);

    let mut user = User{
        name: "Joe".to_string() ,
    };

    let name;
    if user.has_nickname() {
        name = user.nickname();
    } else {
        name = "Unknown person".to_string();
        user.register(&name);
    }

    println!("name: {:?}", name);

    // the first line variable is of type <Result, io:Error>
    // the second line variable is of type String
    // this is very common and is called Shadowing
    // for line in file.lines() {
    //     let line = line?;
    // }

    // use std::io;
    // use std::cmp::Ordering;

    // fn show_files() -> Result<()> {
    //     let mut v = vec![];

    //     fn cmp_by_timestamp_then_name(a: &FileInfo, b: &FileInfo) -> Ordering {
    //         a.timestamp.cmp(&b.timestamp)
    //         .reverse()
    //         .then(a.path.cmp(&b.path))
    //     }

    //     v.sort_by(cmp_by_timestamp_then_name);
    // }
}
