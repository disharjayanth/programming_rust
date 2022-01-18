struct S<'a> {
    r: &'a i32,
}

struct D<'a> {
    s: S<'a>,
}

fn main() {
    let s;
    {
        let x = 10;
        s = S { r: &x };
        assert_eq!(*s.r, 10);
    }
}
