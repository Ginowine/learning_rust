use test_private_functions;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_private_functions::add_two(2));
}