use std::{cmp::Ordering, collections::VecDeque};

#[derive(Debug, Clone)]
pub struct BinarySearchTree<T>
where
    T: Ord,
{
    value: Option<T>,
    left: Option<Box<BinarySearchTree<T>>>,
    right: Option<Box<BinarySearchTree<T>>>,
}

impl<T> Default for BinarySearchTree<T>
where
    T: Ord,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        BinarySearchTree {
            value: None,
            left: None,
            right: None,
        }
    }

    pub fn search(&self, value: &T) -> bool {
        self.value
            .as_ref()
            .map_or(false, |key| match key.cmp(value) {
                Ordering::Equal => true,
                Ordering::Greater => self.left.as_ref().map_or(false, |node| node.search(value)),
                Ordering::Less => self.right.as_ref().map_or(false, |node| node.search(value)),
            })
    }

    pub fn insert(&mut self, value: T) {
        if self.value.is_none() {
            self.value = Some(value)
        } else {
            let key = self.value.as_ref().unwrap();

            let target_node = if *key > value {
                &mut self.left
            } else {
                &mut self.right
            };

            match target_node {
                Some(ref mut node) => {
                    node.insert(value);
                }
                None => {
                    let mut node = Self::new();
                    node.insert(value);
                    *target_node = Some(Box::new(node));
                }
            }
        }
    }

    pub fn minimum(&self) -> Option<&T> {
        self.left
            .as_ref()
            .map_or(self.value.as_ref(), |node| node.minimum())
    }

    pub fn maximum(&self) -> Option<&T> {
        self.right
            .as_ref()
            .map_or(self.value.as_ref(), |node| node.maximum())
    }

    pub fn floor(&self, value: &T) -> Option<&T> {
        let key = self.value.as_ref()?;

        match key.cmp(value) {
            Ordering::Equal => Some(key),
            Ordering::Less => self
                .right
                .as_ref()
                .map_or(Some(key), |node| node.floor(value).or(Some(key))),
            Ordering::Greater => self.left.as_ref().map_or(None, |node| node.floor(value)),
        }
    }

    pub fn ceil(&self, value: &T) -> Option<&T> {
        let key = self.value.as_ref()?;

        match key.cmp(value) {
            Ordering::Equal => Some(key),
            Ordering::Less => self.right.as_ref().map_or(None, |node| node.ceil(value)),
            Ordering::Greater => self
                .left
                .as_ref()
                .map_or(Some(key), |node| node.ceil(value).or(Some(key))),
        }
    }

    pub fn len(&self) -> usize {
        self.iter().collect::<Vec<_>>().len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        BinarySearchTreeIterator::new(self)
    }

    fn values<'a>(&'a self, vs: &mut VecDeque<&'a T>) {
        if self.left.is_some() {
            self.left.as_ref().unwrap().values(vs);
        }

        if self.value.is_some() {
            vs.push_back(self.value.as_ref().unwrap());
        }

        if self.right.is_some() {
            self.right.as_ref().unwrap().values(vs);
        }
    }
}

impl<T> From<Vec<T>> for BinarySearchTree<T>
where
    T: Ord,
{
    fn from(value: Vec<T>) -> Self {
        let mut tree = Self::new();

        for v in value {
            tree.insert(v);
        }

        tree
    }
}

struct BinarySearchTreeIterator<'a, T> {
    values: VecDeque<&'a T>,
}

impl<'a, T> BinarySearchTreeIterator<'a, T>
where
    T: Ord,
{
    fn new(tree: &'a BinarySearchTree<T>) -> Self {
        let mut vs = VecDeque::new();

        tree.values(&mut vs);

        BinarySearchTreeIterator { values: vs }
    }
}

impl<'a, T> Iterator for BinarySearchTreeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.values.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_data() -> (BinarySearchTree<usize>, Vec<usize>, Vec<usize>) {
        let vs = vec![5, 1, 4, 4, 4, 6, 5, 4, 5, 6, 5, 9, 7, 6];
        let copy = vs.clone();
        let mut sorted = vs.clone();
        sorted.sort();

        (BinarySearchTree::from(copy), vs, sorted)
    }

    #[test]
    fn iter() {
        let (tree, _, sorted) = get_data();
        // should iter through the items in ascending order
        let v = tree.iter().map(|v| *v).collect::<Vec<usize>>();

        println!("v = {:?}", v);

        assert_eq!(v, sorted);
    }

    #[test]
    fn len() {
        let (tree, vs, _) = get_data();

        assert_eq!(tree.len(), vs.len());
    }

    #[test]
    fn search() {
        let (tree, vs, _) = get_data();

        for v in &vs {
            assert_eq!(tree.search(v), true);
        }

        let falsy = [10, 13, 15, 100];

        for v in &falsy {
            assert_eq!(tree.search(v), false);
        }
    }

    #[test]
    fn minimum() {
        let (tree, vs, _) = get_data();

        assert_eq!(tree.minimum(), vs.iter().min());
    }

    #[test]
    fn maximum() {
        let (tree, vs, _) = get_data();

        assert_eq!(tree.maximum(), vs.iter().max());
    }

    #[test]
    fn floor() {
        let (tree, vs, _) = get_data();

        assert_eq!(tree.floor(&100), Some(&9));
        assert_eq!(tree.floor(&0), None);
        assert_eq!(tree.floor(&2), Some(&1));
        assert_eq!(tree.floor(&8), Some(&7));

        for v in &vs {
            // tree.floor should return the same value if it exists in the tree
            assert_eq!(tree.floor(v), Some(v));
        }
    }
    #[test]
    fn ceil() {
        let (tree, vs, _) = get_data();

        assert_eq!(tree.ceil(&100), None);
        assert_eq!(tree.ceil(&10), None);
        assert_eq!(tree.ceil(&0), Some(&1));
        assert_eq!(tree.ceil(&3), Some(&4));
        assert_eq!(tree.ceil(&8), Some(&9));

        for v in &vs {
            // tree.ceil should return the same value if it exists in the tree
            assert_eq!(tree.ceil(v), Some(v));
        }
    }
}
