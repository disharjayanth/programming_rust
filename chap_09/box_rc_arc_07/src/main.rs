use std::rc::Rc;

struct Node {
    tag: String,
    children: Vec<Rc<Node>>,
}

impl Node {
    fn new(tag: &str) -> Node {
        Node {
            tag: tag.to_string(),
            children: vec![],
        }
    }

    fn append_to(self: Rc<Self>, parent: &mut Node) {
        parent.children.push(self);
    }
}

fn main() {
    let shared_node = Rc::new(Node::new("first"));
    // shared_node.append_to(&mut parent);
}
