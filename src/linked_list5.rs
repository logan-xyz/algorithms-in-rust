use std::{
    marker::PhantomData,
    ptr::{self, NonNull},
};

pub struct LinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<T>,
}

type Link<T> = Option<ptr::NonNull<Node<T>>>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T,
}

pub struct IntoIter<T> {
    list: LinkedList<T>,
}

pub struct Iter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a mut T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));

            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                self.back = Some(new);
            }

            self.front = Some(new);
            self.len += 1;
        };
    }
    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                back: None,
                front: None,
                elem,
            })));

            if let Some(old) = self.back {
                (*old.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(old);
            } else {
                self.front = Some(new);
            }

            self.back = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            let node = self.front?;

            let boxed_node = Box::from_raw(node.as_ptr());
            let result = boxed_node.elem;

            self.front = boxed_node.back;

            if let Some(new) = self.front {
                (*new.as_ptr()).front = None;
            } else {
                self.back = None
            }

            self.len -= 1;
            Some(result)
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.back.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                self.back = boxed_node.front;
                if let Some(new) = self.back {
                    (*new.as_ptr()).back = None;
                } else {
                    self.front = None;
                }

                self.len -= 1;
                result
            })
        }
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { Some(&self.front?.as_ref().elem) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { Some(&mut self.front?.as_mut().elem) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe {
            self.back
                .map(|node| &(*node.as_ptr()).elem)
        }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.back
                .map(|node| &mut (*node.as_ptr()).elem)
        }
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            back: self.back,
            front: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }

    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter { list: self }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() {}
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type IntoIter = IntoIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a LinkedList<T> {
    type IntoIter = Iter<'a, T>;
    type Item = &'a T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut LinkedList<T> {
    type IntoIter = IterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let node = self.front?;

            self.len -= 1;

            unsafe {
                self.front = (*node.as_ptr()).back;
                Some(&(*node.as_ptr()).elem)
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            let node = self.back?;

            self.len -= 1;

            unsafe {
                self.back = (*node.as_ptr()).front;
                Some(&(*node.as_ptr()).elem)
            }
        } else {
            None
        }
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<T> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn test_basic_front() {
        let mut list = LinkedList::new();

        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }
}
