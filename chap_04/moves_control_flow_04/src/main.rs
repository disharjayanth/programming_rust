fn main() {
    let x = vec![10, 20, 30, 40];
    let c = true;
    if c {
        // f(x); ok to move x from here;
    } else {
        // g(x); and also ok to move x from here;
    }
    // h(x); !!!! bad:x is uninitialized here if either path uses it

    // // while f() {
    // //      g(x); bad x would be moved in first iteration ,
    // //      and unitialised in second
    // // }

    // // unless we've definitely given it a new value by next iteration
    // let mut x = vec![10, 20, 30];
    // while f() {
    //     g(x); // move from x
    //     x = h(); // give x a fresh value
    // }
    // e(x);
}
