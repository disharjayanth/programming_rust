fn main() {
    // let mut x = 10;
    // let r1 = &x;
    // let r2 = &x;     // ok: multiple shared borrows permitted
    // x += 10;         // error: cannot assign to `x` because it is borrowed
    // let m = &mut x;  // error: cannot borrow `x' as mutable because it is
                        // also borrowed as immutable

    // println!("{}, {}, {}", r1, r2, m); 

    // let mut y = 20;
    // let m1 = &mut y;
    // let m2 = &mut y; // error: cannot borrow as mutable more than once
    // let z = y;           // error: cannot use `y` because it was mutably borrowed
    // println!("{}, {}, {}", m1, m2, z);

    let mut v = (136, 139);
    let m = &mut v;
    let m0 = &mut m.0; // ok: reborrowing mutable from mutable
    *m0 = 140;                  
    let r1 = &m.1; // ok: reborrowing shared from mutable
                        // and does'nt overlap with m0

    // v.1;                // error: access through other paths still forbidden
    println!("r1: {}", r1);                    
}
