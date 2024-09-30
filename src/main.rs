struct Node {
    id: i32,
    content: String,
    parent: Option<Box<Node>>
}

fn main() {
    let node1 = Node {
        id: 0,
        content: "Test".into(),
        parent: None,
    };

    println!("{}", node1.content);
}
