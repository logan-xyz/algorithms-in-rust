use std::ptr;

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail = Box::into_raw(Box::new(Node {
            elem,
            next: ptr::null_mut(),
        }));

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = new_tail;
            }
        } else {
            self.head = new_tail;
        }

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_null() {
            None
        } else {
            let head = unsafe { Box::from_raw(self.head) };
            self.head = head.next;

            if head.next.is_null() {
                self.tail = ptr::null_mut();
            }

            Some(head.elem)
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T> {
        if self.head.is_null() {
            Iter { next: None }
        } else {
            Iter {
                next: unsafe { Some(&*self.head) },
            }
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        if self.head.is_null() {
            IterMut { next: None }
        } else {
            IterMut {
                next: unsafe { Some(&mut *self.head) },
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
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
        match self.next.take() {
            None => None,
            Some(node) => {
                if !node.next.is_null() {
                    self.next = unsafe { Some(&*node.next) };
                }

                Some(&node.elem)
            },
        }
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.take() {
            None => None,
            Some(node) => {
                if !node.next.is_null() {
                    self.next = unsafe { Some(&mut *node.next) };
                }

                Some(&mut node.elem)
            },
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
}
