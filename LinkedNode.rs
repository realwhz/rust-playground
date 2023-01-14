#[allow(dead_code)]
#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node { value, next: None }
    }

    fn append(&mut self, value: i32) {
        let mut curr = self;
        while let Some(ref mut x) = curr.next {
            curr = x;
        }
        curr.next = Some(Box::new(Self::new(value)));
    }

    fn prepend(&mut self, value: i32) {
        let old_val = self.value;
        let old_tail = self.next.take();
        self.next = Some(Box::new(Node {
            value: old_val,
            next: old_tail,
        }));
        self.value = value;
    }
}

fn main() {
    let mut n = Node::new(12);
    n.append(13);
    n.append(14);
    n.prepend(11);
    n.append(15);
    n.prepend(10);
    println!("{:#?}", n);
}
