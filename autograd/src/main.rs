use std::rc::Rc;

#[derive(Debug)]
enum Op {
    Add,
}

#[derive(Debug)]
struct Tensor {
    id: usize,
    op: Option<Op>,
    inputs: Vec<Rc<Tensor>>,
}

impl Tensor {
    fn leaf(id: usize) -> Rc<Self> {
        Rc::new(Tensor { id, op: None, inputs: vec![] })
    }
}

fn add(a: Rc<Tensor>, b: Rc<Tensor>, id: usize) -> Rc<Tensor> {
    Rc::new(Tensor {
        id,
        op: Some(Op::Add),
        inputs: vec![a, b],
    })
}

fn main() {
    let x = Tensor::leaf(0);
    let y = Tensor::leaf(1);
    let z = add(x, y, 2);

    println!("{:#?}", z);
}
