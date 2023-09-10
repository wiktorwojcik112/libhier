#![allow(unreachable_code)]

extern crate core;

use std::fs;
use libhier;
use libhier::hier::Hier;

fn exit_handler() -> ! {
    panic!("")
}
fn module_reader(path: String) -> String {
    fs::read_to_string(path.clone())
        .expect(&format!("Unable to read the file: {}", path))
}

#[test]
fn map_function_works() {
    let mut hier = Hier::new(String::new(), |_| { String::new() }, exit_handler);

    let value = hier.run("(map (1 2 3) { (+ element 1) })".to_string());

    assert_eq!(value.text_representation(), "2 3 4 ");
}

#[test]
fn pipe_operator_works() {
    let mut hier = Hier::new(String::new(), |_| { String::new() }, exit_handler);

    let value = hier.run("(1 2 3) > (map { (+ element 1) }) > (get)".to_string());

    assert_eq!(value.text_representation(), "2 3 4 ");
}

#[test]
// Test whether object.function syntax works properly.
fn does_object_root_work() {
    let mut hier = Hier::new(String::new(), |_| { String::new() }, exit_handler);

    let value = hier.run("((1 2 3).map { (+ element 1) })".to_string());

    assert_eq!(value.text_representation(), "2 3 4 ");
}

#[test]
fn does_import_work() {
    let mut hier = Hier::new("./general_tests.rs".to_string(), module_reader, exit_handler);

    let value = hier.run("run { (@test (import \"tests/testing_module\")) (test#hello \"World\") (get test#pi) }".to_string());

    assert_eq!(value.text_representation(), "3.14159265359");
}

#[test]
fn does_automatic_root_parenthesis_work() {
    let mut hier = Hier::new(String::new(), |_| { String::new() }, exit_handler);

    let value = hier.run("get (+ 2 2)".to_string());

    assert_eq!(value.text_representation(), "4");
}

