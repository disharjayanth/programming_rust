use std::collections::btree_map::Values;

fn f() {
    println!("f fn")
}

fn g() {
    println!("g fn")
}

fn ff() -> Option<i64> {
    return Some(42);
}

fn h(x: &i64) {
    println!("{}", x);
}

fn j() -> i32 {
    return 0;
}

#[derive(Debug)]
struct Point {
    x: i32 ,
    y: i32
}

impl Point {
    fn translate(&mut self, x: i32, y: i32) {
        println!("x: {}, y: {}", x, y);
        self.x = x;
        self.y = y;
    }
}

fn main() {
    // Expression types
    // Array literal
    let a = [1, 2, 3];
    // Repeat array literal
    let b = [0; 50];
    // Tuple
    let c = (6, "crullers");

    println!("a is array of 3 elements :{:?}", a);
    println!("b is repeated array of 50 elements with 0 value: {:?}", b);
    println!("c is a tuple of i32 and string: {:?}", c);

    // Grouping
    println!("Grouping: {}", (2+2));
    // Block
    {
        println!("calling f and g fn from block");
        f();
        g();
    }

    // Control flow expressions
    let ok = true;
    if ok {
        println!("Calling f if ok is true");
        f();
    }

    let not_ok = false;
    if not_ok {
        println!("not_ok is true");
        f();
    } else {
        println!("not_ok is false");
        g()
    }

    if let Some(x) = ff() { 
        println!("value of x: {}", x); 
    } else {
        println!("else: {}", 0); 
    };

    // let x = 10;
    // match x {
    //     None => 0 ,
    //     _ => 1
    // };

    let d = [1, 2, 3, 4, 5, 10];
    for v in d {
        h(&v);
    }

    // infinte loop
    // while ok {
        // ok = f();
       // println!("{}", ok);
    // }

    // while let Some(x) = it.next() {
    //     f();
    // }

    // infinite loop until break
    let mut i = 0;
    loop {
        i += 1;
        if i == 5 {
            break;
        }
        f();
    }

    loop {
        i += 1;
        println!("{}", i);
        if i == 5 {
            continue;
        } 
        if i == 10 {
            break;
        }
    }

    println!("Calling j fn: {}", j());

    // macro invokation
    println!("macro invokation ok");

    println!("PI from std: {}", std::f64::consts::PI);
    
    let pair = (1.1, 2.2);
    println!("pair tuple: {}", pair.0);

    let mut p = Point{
        x: 0 ,
        y: 0
    };

    println!("{:?} and accessing individual fields x: {} and y: {}", p, p.x, p.y);
    p.translate(10, 12);
    println!("{:?} and accessing individual fields x: {} and y: {}", p, p.x, p.y);

    // function call
    // stdin()

    // Index
    // arr[0]

    // error check
    // create_dir("tmp")?

    // logical/bitwise NOT
    // !ok

    // negation
    // -num

    // deference
    // *ptr

    // borrow
    // &val

    // type cast
    // x as u32 
    let x = 10 as u32;
    println!("x: {}", x);

    // simple math operations
    let e = 10;
    println!("multiplication: {}", e * 2);
    println!("division: {}", e / 2);
    println!("modulus: {}", e % 2);
    println!("addition: {}", e + 2);
    println!("substraction: {}", e - 3);

    let n = 20;
    println!("left shift: {}", n << 1);
    println!("right shift: {}", n >> 1);

    println!("Bitwise AND: {}", n & 1);
    println!("Bitwise exclusive OR: {}", n ^ 1);
    println!("Bitwise OR: {}", n | 1);

    println!("Less than: {}", n < 1);
    println!("Less than or equal to: {}", n <= 1);
    println!("Greater than: {}", n > 1);
    println!("Greater than or equal: {}", n >= 1);
    println!("Equal: {}", n == 20);
    println!("Not equal: {}", n != 22);

    struct S {
        x: bool ,
        y: bool
    }

    struct T {
        x: bool ,
        y: bool
    }

    let s = S{
        x: true ,
        y: false
    };

    let t = T{
        x: false ,
        y: true 
    };

    println!("Logical AND: {}", s.x && t.x);    
    println!("Logical OR: {}", s.y || t.y);

    // End-exclusive range:
    // start .. stop

    // End-inclusive range:
    // start ..= stop

    // Compound Assignments
    // x = val 
    // x *= 1
    // x /= 1
    // x %= 1
    // x += 1
    // x -= 1
    // x <<= 1
    // x >>= 1
    // x &= 1
    // x ^= 1
    // x |= 1

    // Closure
    // |x, y| x + y
}
