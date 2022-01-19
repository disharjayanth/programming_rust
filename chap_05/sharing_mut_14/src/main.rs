fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

fn main() {
    // below code throws error since v(vector) is moved to aside
    // and still getting borrowed at r[0]
    // let v = vec![4, 8, 19, 27, 34, 10];
    // let r = &v;
    // let aside = v;
    // r[0];

    // below code is correct since r[0] is accessed 
    // before moving it to aside
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0];
        assert_eq!(r[0], 4);
    }

    let aside = v;
    println!("{:?}", aside);

    let mut wave = Vec::new();
    let head = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, [0.0, 1.0, 0.0, -1.0]);

    extend(&mut wave, &wave);
}
