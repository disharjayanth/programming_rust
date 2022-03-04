#![allow(non_camel_case_types)]
// #![feature(trace_macros)] // can only be used on experimental version

#[derive(Debug)]
pub struct git_revspec {}

#[derive(Debug)]
pub struct git_error {}

fn main() {
    let some_git_revspec = git_revspec {};
    let some_git_error = git_error {};

    println!("{:?}", some_git_revspec);
    println!("{:?}", some_git_error);

    // trace_macros!(true);
    // println!("Hello, world!");
    // trace_macros!(false);
}
