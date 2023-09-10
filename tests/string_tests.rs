#![allow(unreachable_code)]

extern crate core;

use libhier;
use libhier::hier::Hier;

fn exit_handler() -> ! {
    panic!("")
}

#[test]
fn escape_sequences_are_resolved() {
    let mut hier = Hier::new(String::new(),|_| { String::new() }, exit_handler);

    let value = hier.run("(get \"Test \\n\")".to_string());

    assert_eq!(value.text_representation(), "Test \n");
}

#[test]
fn interpolations_are_resolved() {
    let mut hier = Hier::new(String::new(),|_| { String::new() }, exit_handler);

    let value = hier.run("(run (@test 5) (get \"Test \\(get test)\"))".to_string());

    assert_eq!(value.text_representation(), "Test 5");
}

#[test]
fn is_concatenating_properly() {
    let mut hier = Hier::new(String::new(),|_| { String::new() }, exit_handler);

    let value = hier.run("(+ \"Test\" \" concatenation\")".to_string());

    assert_eq!(value.text_representation(), "Test concatenation");
}