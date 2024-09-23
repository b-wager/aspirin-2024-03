// Now try and implement a doubly linked version. Give an explanation
// for why this doesn't work.

struct Node {
    val: i32,
    next: Link,
    prev: Link,
}

type Link = Option<Box<Node>>;

pub struct LinkedStack {
    head: Link,
    tail: Link,
}

impl LinkedStack {
    fn new() -> Self {
        LinkedStack {
            head: None,
            tail: None,
        }
    }

    // fn push(&mut self, val: i32) {
    //     if self.head.is_none() {
    //         let new_node = Box::new(Node {
    //             val,
    //             next: None,
    //             prev: None,
    //         });
    //         self.head = Some(new_node);
    //         self.tail = Some(new_node);
    //     } else {
    //         let new_node = Box::new(Node {
    //             val,
    //             next: self.head.take(),
    //             prev: None,
    //         });
    //         self.head.as_mut().map(|node| node.prev = Some(new_node));
    //         self.head = Some(new_node);
    //     }
    // }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            if let Some(node) = self.head.as_mut() {
                node.prev = None;
            }
            node.val
        })
    }
}
