struct Tensor;

#[derive(Debug)]
enum Node {
    Leaf,
    Add(Box<Node>, Box<Node>),
}

fn leaf(_t: Tensor) -> Node {
    Node::Leaf
}

fn add(a: Node, b: Node) -> Node {
    Node::Add(Box::new(a), Box::new(b))
}

fn main() {
    let x = leaf(Tensor);
    let y = leaf(Tensor);
    let z = add(x, y);

    println!("{:#?}", z);
}
