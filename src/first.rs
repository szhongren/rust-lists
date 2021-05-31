// List a = Empty | Elem a (List a)

pub enum List {
    Empty,
    Elem(i32, Box<List>),
}

// small issue, first element will be on the stack
// [] = Stack
// () = Heap

// [Elem A, ptr] -> (Elem B, ptr) -> (Empty, *junk*)
