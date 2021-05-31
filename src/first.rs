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

    // pub fn pop(&mut self) -> Option<i32> {
    //     match self.head {
    //         Link::Empty => {}
    //         Link::More(node) => {}
    //     };
    //     unimplemented!()
    // }
    // cannot move out of borrowed content, consider borrowing here: `&self.head`

    // pub fn pop(&mut self) -> Option<i32> {
    //     let result;
    //     match &self.head {
    //         Link::Empty => {
    //             result = None;
    //         }
    //         Link::More(ref node) => {
    //             result = Some(node.elem); // elem is copied here
    //             self.head = node.next;
    //         }
    //     };
    //     result
    // }
    // cannot move out of `node.next` which is behind a shared reference

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
        // The key insight is we want to remove things,
        // which means we want to get the head of the list by value.
        // We certainly can't do that through the shared reference we get through &self.head.
        // We also "only" have a mutable reference to self, so the only way we can move stuff is to replace it.
        // Looks like we're doing the Empty dance again!
    }
}

// impl Drop for List {
//     fn drop(&mut self) {
//         // NOTE: you can't actually explicitly call `drop` in real Rust code;
//         // we're pretending to be the compiler!
//         self.head.drop(); // tail recursive - good!
//     }
// }

// impl Drop for Link {
//     fn drop(&mut self) {
//         match *self {
//             Link::Empty => {} // Done!
//             Link::More(ref mut boxed_node) => {
//                 boxed_node.drop(); // tail recursive - good!
//             }
//         }
//     }
// }

// impl Drop for Box<Node> {
//     fn drop(&mut self) {
//         self.ptr.drop(); // uh oh, not tail recursive!
//         deallocate(self.ptr);
//     }
// }

// impl Drop for Node {
//     fn drop(&mut self) {
//         self.next.drop();
//     }
// }

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        // TODO
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
