use std::{
    cell::{Ref, RefCell, RefMut},
    mem,
    rc::Rc,
};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    prev: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    // five steps to push an elephant into freezer
    pub fn push_front(&mut self, elem: T) {
        // step 1: create a new node
        let new_node = Node::new(elem);

        // step 2: make the list head pointing to the new_node
        let old_head = mem::replace(&mut self.head, Some(new_node.clone()));

        // step 3: make the new_now.next pointing to the new_node
        new_node.borrow_mut().next = old_head.clone();

        match old_head {
            // step 4: if the old_head is not None, it's prev pointer should now pointing to the new_node
            Some(node) => {
                node.borrow_mut().prev = Some(new_node);
            },
            // step 5: if the old_head is None, the list tail pointer should now pointing to the new_node
            None => {
                self.tail = Some(new_node);
            },
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_node = Node::new(elem);
        let old_tail = mem::replace(&mut self.tail, Some(new_node.clone()));

        new_node.borrow_mut().prev = old_tail.clone();

        match old_tail {
            Some(node) => {
                node.borrow_mut().next = Some(new_node);
            },
            None => {
                self.head = Some(new_node);
            },
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        // step 1: take the old_head off the list. If the list is empty, return None instead.
        self.head.take().map(|old_head| {
            // step 2: take the new_head out of the old_head
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // step 3: make the prev pointer of the new_head point to None
                    new_head.borrow_mut().prev.take();
                    // step 4: make the list head pointing to the  new_head
                    self.head = Some(new_head);
                },
                None => {
                    // step 5: set the list tail to None if there's no new head
                    self.tail.take();
                },
            }

            // step 6: unwrap and take elem out of the old_head and return it
            Rc::try_unwrap(old_head)
                .ok()
                .unwrap()
                .into_inner()
                .elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                },
                None => {
                    self.head.take();
                },
            }

            Rc::try_unwrap(old_tail)
                .ok()
                .unwrap()
                .into_inner()
                .elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_front_mut(&self) -> Option<RefMut<T>> {
        self.head
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_back_mut(&self) -> Option<RefMut<T>> {
        self.tail
            .as_ref()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }
}
