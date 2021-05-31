// List a = Empty | Elem a (List a)

#[derive(Debug)]
pub enum List1 {
    Empty,
    Elem(i32, Box<List1>),
}

// small issue, first element will be on the stack
// [] = Stack
// () = Heap

// [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)

// really bad idea
pub enum List2 {
    Empty,
    ElemThenEmpty(i32),
    ElemThenNotEmpty(i32, Box<List2>),
}

// need everything to be pub
pub struct Node1 {
    elem: i32,
    next: List3,
}

pub enum List3 {
    Empty,
    More(Box<Node1>),
}

// final implementation
pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}
