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

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
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
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
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

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
