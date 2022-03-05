#[allow(unused_imports)]
use std::num::ParseIntError;

#[test]
fn math_works() {
    let x: i32 = 1;
    assert!(x.is_positive());
    assert_eq!(x + 1, 2);
}

#[test]
fn string_works() {
    let x = "hello world";
    assert_eq!(x, "hello world");
}

#[test]
fn check_bool() {
    let x = true;
    assert_eq!(true, x);
}

// below test check_num will fail if x != 12
#[test]
fn check_num() {
    let x = 12;
    assert_eq!(12, x);
}

#[test]
#[should_panic(expected = "divide by zero")]
#[allow(unconditional_panic, unused_must_use)]
fn test_divide_by_zero_error() {
    1 / 0;
}

#[test]
fn parse() -> Result<(), ParseIntError> {
    i32::from_str_radix("1024", 10)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    fn roughly_equal(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-6
    }

    #[test]
    fn trig_works() {
        use std::f64::consts::PI;
        assert!(roughly_equal(PI.sin(), 0.0))
    }
}

fn main() {}
