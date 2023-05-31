pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elme: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Some(Box::new(Node {
            elme: elem,
            next: self.head.take(),
        }));

        self.head = new_node;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(node) => {
                self.head = node.next;
                Some(node.elme)
            },
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head
            .as_ref()
            .map(|node| &node.elme)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head
            .as_mut()
            .map(|node| &mut node.elme)
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    // pub fn iter<'a>(&'a self) -> Iter<'a, T> {
    //     // first try
    //     // let next = self.head.as_ref().map(|node| &**node);
    //     // Iter { next }
    //     //
    //     // slily better
    //     // let next = self.head.as_ref().map::<&Node<T>, _>(|node| node);
    //     // Iter { next }
    //
    //     // finally
    //     Iter {
    //         next: self.head.as_deref(),
    //     }
    // }

    // lifetime elision
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    // if you're not comfortable "hiding" that a struct contains a lifetime, you can use the
    // Rust 2018 "explicitly elided lifetime" syntax, '_
    //
    // pub fn iter(&self) -> Iter<'_, T> {
    //     Iter {
    //         next: self.head.as_deref(),
    //     }
    // }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // map takes self as its' first argument but since & is copy, Option<&> is also Copy.
        // so Option<&T> is copied instead of moved and self remains to be valid after calling self.next.take
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elme
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

// impl<'a, T> Iterator for IterMut<'a, T> {
//     type Item = &'a mut T;
//     fn next(&mut self) -> Option<Self::Item> {
//         // map takes self as its' first argument. &mut is not copy, and Option<&mut> is not Copy too.
//         // (if you copied an &mut, you'd have two &mut's to the same location in memory, which is forbidden)
//         // So Option<&mut T> will be moved out of self after calling self.next.map, which are not allowed.
//         // you can't move some apart it from a either mutable or immutable reference.
//         self.next.map(|node| {
//             self.next = node.next.as_deref_mut();
//             &mut node.elme
//         })
//     }
// }

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elme
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.pop(), Some(2));

        list.push(4);
        list.push(5);

        assert_eq!(list.peek(), Some(&5));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.peek(), Some(&4));
        assert_eq!(list.pop(), Some(4));

        assert_eq!(list.peek(), Some(&1));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.peek(), None);
        assert_eq!(list.pop(), None);
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

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), None);
    }
}
