use std::{collections::HashMap, hash::Hash};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
struct Node<K, V>
where
    K: Default + Hash + Eq,
    V: Default,
{
    children: HashMap<K, Node<K, V>>,
    value: Option<V>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Trie<K, V>
where
    K: Default + Eq + Hash,
    V: Default,
{
    root: Node<K, V>,
}

impl<K, V> Trie<K, V>
where
    K: Default + Eq + Hash,
    V: Default,
{
    pub fn new() -> Self {
        Self {
            root: Node::default(),
        }
    }

    pub fn insert(&mut self, key: impl IntoIterator<Item = K>, value: V)
    where
        K: Eq + Hash,
    {
        let mut node = &mut self.root;

        for c in key.into_iter() {
            node = node.children.entry(c).or_insert_with(Node::default);
        }

        node.value = Some(value);
    }

    pub fn get(&self, key: impl IntoIterator<Item = K>) -> Option<&V>
    where
        K: Eq + Hash,
    {
        let mut node = &self.root;

        for c in key.into_iter() {
            if node.children.contains_key(&c) {
                node = node.children.get(&c).unwrap();
            } else {
                return None;
            }
        }

        node.value.as_ref()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut trie = Trie::new();

        assert_eq!(trie.get("".chars()), None);
        assert_eq!(trie.get("abc".chars()), None);

        trie.insert("bar".chars(), 5);
        trie.insert("barz".chars(), 10);
        trie.insert("bark".chars(), 20);

        assert_eq!(trie.get("ba".chars()), None);
        assert_eq!(trie.get("bar".chars()), Some(&5));
        assert_eq!(trie.get("barz".chars()), Some(&10));
        assert_eq!(trie.get("bark".chars()), Some(&20));

        // print the json representation of trie structure
        //
        // {
        //   "root": {
        //     "children": {
        //       "b": {
        //         "children": {
        //           "a": {
        //             "children": {
        //               "r": {
        //                 "children": {
        //                   "k": {
        //                     "children": {},
        //                     "value": 20
        //                   },
        //                   "z": {
        //                     "children": {},
        //                     "value": 10
        //                   }
        //                 },
        //                 "value": 5
        //               }
        //             },
        //             "value": null
        //           }
        //         },
        //         "value": null
        //       }
        //     },
        //     "value": null
        //   }
        // }
        println!("{}", serde_json::to_string_pretty(&trie).unwrap());
    }
}
