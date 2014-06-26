extern crate avl_tree;

#[test]
fn test_sanity() {
    let x = 2;
    assert!(x == 2);
}

fn main() {
    println!("hello {}", avl_tree::foo());
}