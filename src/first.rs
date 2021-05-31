use std::mem;

// List a = Empty | Elem a (List a)

mod bad1 {

    #[derive(Debug)]
    pub enum List1 {
        Empty,
        Elem(i32, Box<List1>),
    }

    // small issue, first element will be on the stack
    // [] = Stack
    // () = Heap

    // [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
}

mod bad2 {
    // really bad idea
    #[derive(Debug)]
    pub enum List2 {
        Empty,
        ElemThenEmpty(i32),
        ElemThenNotEmpty(i32, Box<List2>),
    }

    // need everything to be pub
    #[derive(Debug)]
    pub struct Node1 {
        elem: i32,
        next: List3,
    }

    #[derive(Debug)]
    pub enum List3 {
        Empty,
        More(Box<Node1>),
    }
}

// final implementation
#[derive(Debug)]
pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    // pub fn push(&mut self, elem: i32) {
    //     let new_node = Node {
    //         elem,
    //         next: self.head,
    //     };
    // }
    // cannot move self.head out of borrowed content

    // pub fn push(&mut self, elem: i32) {
    //     let new_node = Box::new(Node {
    //         elem,
    //         next: self.head,
    //     });
    //     self.head = Link::More(new_node);
    // }
    // cannot move self.head out of borrowed content

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}
