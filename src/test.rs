extern crate tree;

#[test]
fn test_sanity() {
    let x = 2;
    assert!(x == 2);
}

#[test]
fn trivial_import_from_lib() {
    assert!(format!("hello {}", tree::avl_tree::foo()) == String::from_str("hello world"));
}