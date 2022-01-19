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

    // below code throws error because in extend fn, vec: &mut Vec<f64> is only way to reach vector 
    // or it's elements, the shared reference from slice: &[f64] is another way of reaching elems
    // which is NOT good, thus violating 2nd rule of MUTABLE ACCESS IS EXCLUSIVE ACESSS.
    // * OR * 
    // rust could also have treated bug as a violation of first rule: since we've shared reference
    // to wave's elements, the elements and the vec itself are read-only. You can't borrow a mutable
    // reference to a read-only value thus violating SHARED ACESS IS READ ONLY ACCESS.
    // extend(&mut wave, &wave);
}
