// fn borrow_variable() {
//     let r: &i32;
//     {
//         let x = 1;
//         r = &x;
//     }
//     assert_eq!(*r, 1);
// }

fn borrow_variable() {
    let x = 1;
    {
        let r = &x;
        assert_eq!(*r, 1);
    }
}

fn main() {
    borrow_variable();

    let v = vec![1, 2, 3];
    let r = &v[1];
    println!("r: {}", r);
}
