use std::rc::Rc;

fn main() {
    let rc_examples = "Rc examples".to_string();

    println!("----rc is created-----");

    let rc_a = Rc::new(rc_examples);

    println!("Reference count of rc_a: {}", Rc::strong_count(&rc_a));

    {
        let rc_b = Rc::clone(&rc_a);

        println!("Reference count of rc_b: {}", Rc::strong_count(&rc_a));
        println!("Reference count of rc_a: {}", Rc::strong_count(&rc_b));

        println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));
        println!("Length of value rc_a: {}", rc_a.len());
        println!("Value of rc_b: {}", rc_b);
    }

    println!("Reference count of rc_a: {}", Rc::strong_count(&rc_a));
}
