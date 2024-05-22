struct Node {
    node: List,
    value: u32,
}

enum List {
    Empty,
    Link(Box<Node>),
}

#[test]
fn lista_enlazada() {
    println!("this is a test");
}