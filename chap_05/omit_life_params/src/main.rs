struct S<'a, 'b> {
    x: &'a i32,
    y: &'b i32,
}

fn sum_r_xy(r: &i32, s: S) -> i32 {
    r + s.x + s.y
}

fn first_third(point: &[i32; 4]) -> (&i32, &i32) {
    (&point[0], &point[2])
}

struct StringTable {
    elements: Vec<String>,
}

impl StringTable {
    fn find_by_prefix(&self, prefix: &str) -> Vec<&String> {
        let mut temp: Vec<&String> = vec![];
        for i in 0..self.elements.len() {
            if self.elements[i].starts_with(prefix) {
                temp.push(&self.elements[i]);
            }
        }
        temp
    }
}

fn main() {
    let r = 10;
    let x = 5;
    let y = 5;
    let s = S { x: &x, y: &y };

    println!("sum_r_xy fn: {}", sum_r_xy(&r, s));

    let point = [1, 2, 3, 4];
    println!("first_third fn: {:?}", first_third(&point));

    let string_table = StringTable {
        elements: vec![
            "Apple".to_string(),
            "Arrow".to_string(),
            "$ansible".to_string(),
            "banana".to_string(),
            "$xaxaxa".to_string(),
        ],
    };

    let prefix = "$";
    println!("find_by_prefix: {:?}", string_table.find_by_prefix(prefix));
}
