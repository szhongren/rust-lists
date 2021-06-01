// final implementation
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    // pub fn peek(&self) -> Option<&T> {
    //     self.head.map(|node| &node.elem)
    // }
    // cannot move out of `self.head` which is behind a shared reference
    // cannot return reference to local data `node.elem`
    // map takes self by value, which will move the Option out, but now we want to leave the Option instead of take()

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

// this is a tuple struct, just wraps around List<T>
pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}

// above is not technically required, but nice to have
// impl<T> Iterator for List<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.pop()
//     }
// }

// Iter is generic over *some* lifetime, it doesn't care
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

// No lifetime here, List doesn't have any associated lifetimes
impl<T> List<T> {
    // We declare a fresh lifetime here for the *exact* borrow that
    // creates the iter. Now &self needs to be valid as long as the
    // Iter is around.
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.head.map(|node| &node),
        }
    }
}

// We *do* have a lifetime here, because Iter has one that we need to define
impl<'a, T> Iterator for Iter<'a, T> {
    // Need it here too, this is a type declaration
    type Item = &'a T;

    // None of this needs to change, handled by the above.
    // Self continues to be incredibly hype and amazing
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.map(|node| &node);
            &node.elem
        })
    }
}

// lifetime elision
// // Only one reference in input, so the output must be derived from that input
// fn foo(&A) -> &B; // sugar for:
// fn foo<'a>(&'a A) -> &'a B;

// // Many inputs, assume they're all independent
// fn foo(&A, &B, &C); // sugar for:
// fn foo<'a, 'b, 'c>(&'a A, &'b B, &'c C);

// // Methods, assume all output lifetimes are derived from `self`
// fn foo(&self, &B, &C) -> &D; // sugar for:
// fn foo<'a, 'b, 'c>(&'a self, &'b B, &'c C) -> &'a D;

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

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
        // list.peek_mut().map(|&mut value| value = 42);
        // The compiler is complaining that value is immutable, but we pretty clearly wrote &mut value; what gives?
        // It turns out that writing the argument of the closure that way doesn't specify that value is a mutable reference.
        // Instead, it creates a pattern that will be matched against the argument to the closure;
        // |&mut value| means "the argument is a mutable reference, but just copy the value it points to into value, please."
        // If we just use |value|, the type of value will be &mut i32 and we can actually mutate the head:
        list.peek_mut().map(|value| *value = 42);

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
