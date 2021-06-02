use std::ptr;

mod bad {
    pub struct List<'a, T> {
        head: Link<T>,
        tail: Option<&'a mut Node<T>>,
    }

    type Link<T> = Option<Box<Node<T>>>;

    struct Node<T> {
        elem: T,
        next: Link<T>,
    }

    impl<'a, T> List<'a, T> {
        pub fn new() -> Self {
            List {
                head: None,
                tail: None,
            }
        }

        // pub fn push(&mut self, elem: T) {
        //     // push onto the 'end' of the list, so new_tail is always and end node
        //     let new_tail = Box::new(Node { elem, next: None });

        //     // swap old tail to point at new tail
        //     let old_tail = mem::replace(&mut self.tail, Some(new_tail));

        //     match old_tail {
        //         Some(mut old_tail) => {
        //             // point old_tail's next to new tail
        //             old_tail.next = Some(new_tail);
        //         }
        //         None => {
        //             self.head = Some(new_tail);
        //         }
        //     }
        // }

        pub fn push(&'a mut self, elem: T) {
            // When you push onto the tail, your next is always None
            let new_tail = Box::new(Node { elem, next: None });

            // Put the box in the right place, and then grab a reference to its Node
            let new_tail = match self.tail.take() {
                Some(old_tail) => {
                    // If the old tail existed, update it to point to the new tail
                    old_tail.next = Some(new_tail);
                    old_tail.next.as_deref_mut()
                }
                None => {
                    // Otherwise, update the head to point to it
                    self.head = Some(new_tail);
                    self.head.as_deref_mut()
                }
            };

            self.tail = new_tail;
        }

        pub fn pop(&'a mut self) -> Option<T> {
            // Grab the list's current head
            self.head.take().map(|head| {
                let head = *head;
                self.head = head.next;

                // If we're out of `head`, make sure to set the tail to `None`.
                if self.head.is_none() {
                    self.tail = None;
                }

                head.elem
            })
        }
    }

    // mod test {
    //     use super::List;
    //     #[test]
    //     fn basics() {
    //         let mut list = List::new();

    //         // Check empty list behaves right
    //         assert_eq!(list.pop(), None);

    //         // Populate list
    //         list.push(1);
    //         list.push(2);
    //         list.push(3);

    //         // Check normal removal
    //         assert_eq!(list.pop(), Some(1));
    //         assert_eq!(list.pop(), Some(2));

    //         // Push some more just to make sure nothing's corrupted
    //         list.push(4);
    //         list.push(5);

    //         // Check normal removal
    //         assert_eq!(list.pop(), Some(3));
    //         assert_eq!(list.pop(), Some(4));

    //         // Check exhaustion
    //         assert_eq!(list.pop(), Some(5));
    //         assert_eq!(list.pop(), None);
    //     }
    // }
}

pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>, // DANGER DANGER
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node { elem, next: None });

        let raw_tail: *mut _ = &mut *new_tail;

        // .is_null checks for null, equivalent to checking for None
        if !self.tail.is_null() {
            unsafe {
                // If the old tail existed, update it to point to the new tail
                (*self.tail).next = Some(new_tail);
            }
        } else {
            // Otherwise, update the head to point to it
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}
