pub struct Node {
    pub left_child: Option<Box<Node>>,
    pub data: u32,
    pub right_child: Option<Box<Node>>
}

impl Node {
    pub fn data_to_str(&self) -> String {
        format!("{}", (*self).data)
    }
}

pub fn foo() -> String {
    String::from_str("world")
}