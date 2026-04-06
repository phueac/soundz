struct Tensor;

#[derive(Debug)]
enum Node {
    Leaf,
    Add(Box<Node>, Box<Node>),
    Mul(Box<Node>, Box<Node>),
    ReLU(Box<Node>),
}

fn leaf(_t: Tensor) -> Node {
    Node::Leaf
}

fn add(a: Node, b: Node) -> Node {
    Node::Add(Box::new(a), Box::new(b))
}

fn mul(a: Node, b: Node) -> Node {
    Node::Mul(Box::new(a), Box::new(b))
}

fn relu(a: Node) -> Node {
    Node::ReLU(Box::new(a))
}

fn main() {
    let x = leaf(Tensor);
    let y = leaf(Tensor);
    let z = relu(add(x, mul(y, leaf(Tensor))));

    println!("{:#?}", z);
}
