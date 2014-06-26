extern crate tree;

use tree::avl_tree::Node;

#[test]
fn test_sanity() {
    let x = 2;
    assert!(x == 2);
}

#[test]
fn trivial_import_from_lib() {
    assert!(format!("hello {}", tree::avl_tree::foo()) == String::from_str("hello world"));
}

#[test]
#[should_fail]
fn node_data_test() {
    let node = Node { left_child: None, data: 1984, right_child: None };
    assert!(node.data_to_str() == String::from_str("1983"));
}