// pub struct SpirderRobot {
//     species: String,
//     web_enabled: bool,
//     leg_devices: [fd::FileDesc; 8],
// }

// pub struct SpiderSenses {
//     robot: Rc<SpirderRobot> ,
//     eyes: [Camera; 32] ,
//     motion: Accel
// }

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

fn main() {
    let mut rc_examples = "Rc examples".to_string();

    let rc_a = Rc::new(rc_examples);
    println!("Reference Count of rc_a: {}", Rc::strong_count(&rc_a));

    let rc_b = Rc::clone(&rc_a);
    println!("Reference Count of rc_b: {}", Rc::strong_count(&rc_b));

    rc_examples = "changes".to_string();

    println!("{}", rc_examples);
    println!("{}", rc_a);
    println!("{}", rc_b);

    let a = Cell::new("example");

    println!("{}", a.get());

    a.set("example changes!");

    println!("{}", a.get());

    let b = RefCell::new("ref cell example");

    let borrowed = b.borrow();
    println!("{}", borrowed);
}
