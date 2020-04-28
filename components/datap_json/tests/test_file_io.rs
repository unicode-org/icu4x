#![feature(std)]

extern crate icu_datap_json;

#[test]
fn test_add() {
    assert_eq!(adder::add(3, 2), 5);
}
