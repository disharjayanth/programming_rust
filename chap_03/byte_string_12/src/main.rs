fn main() {
    let method = b"GET";
    println!("{:?}", method);
    assert_eq!(method, &[b'G', b'E', b'T']);

    let raw_byte_slice = br##"Hello world!"##;
    println!("{:?}", raw_byte_slice);
}
