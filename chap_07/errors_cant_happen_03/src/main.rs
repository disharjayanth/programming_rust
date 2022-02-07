fn main() {
    let a = "1234";
    let res = a.parse::<u64>();
    // match res {
    //     Ok(num) => {
    //         println!("{}", &num)
    //     }
    //     Err(err) => {
    //         println!("Parse error: {}", err)
    //     }
    // }

    println!("{}", res.expect("error"));
}
