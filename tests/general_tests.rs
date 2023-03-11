#![allow(unreachable_code)]

extern crate core;

use libhier;
use libhier::hier::Hier;

fn exit_handler() -> ! {
    panic!("")
}

#[test]
fn map_function_works() {
    let mut hier = Hier::new(|_| { String::new() }, exit_handler);

    let value = hier.run("(map (1 2 3) { (+ element 1) })".to_string());

    assert_eq!(value.text_representation(), "2 3 4 ");
}

#[test]
fn pipe_operator_works() {
    let mut hier = Hier::new(|_| { String::new() }, exit_handler);

    let value = hier.run("(1 2 3) > (map { (+ element 1) }) > (get)".to_string());

    assert_eq!(value.text_representation(), "2 3 4 ");
}