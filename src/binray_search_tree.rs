// implement binary search tree in rust

use std::cmp::Ordering::*;

pub struct Node<K: Ord> {
    key: K,
    left: BST<K>,
    right: BST<K>,
}

pub struct BST<K: Ord>(Option<Box<Node<K>>>);

impl<K: Ord> BST<K> {
    pub fn new() -> Self {
        BST(None)
    }

    pub fn insert(&mut self, key: K) -> bool {
        unsafe {
            let mut tree: *mut BST<K> = self;

            while let Some(ref mut node) = (*tree).0 {
                match key.cmp(&node.key) {
                    Less => tree = &mut node.left,
                    Greater => tree = &mut node.right,
                    Equal => return false,
                }
            }

            (*tree).0 = Some(Box::new(Node {
                key,
                left: BST(None),
                right: BST(None),
            }));
            true
        }
    }

    pub fn remove(&mut self, key: K) -> bool {
        let mut tree: *mut BST<K> = self;

        unsafe {
            while let Some(ref mut node) = (*tree).0 {
                match key.cmp(&node.key) {
                    Less => tree = &mut node.left,
                    Greater => tree = &mut node.right,
                    Equal => {
                        (*tree).0 = None;
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn contains(&self, key: K) -> bool {
        let mut tree = self;

        while let Some(ref node) = tree.0 {
            match key.cmp(&node.key) {
                Less => tree = &node.left,
                Greater => tree = &node.right,
                Equal => return true,
            }
        }

        false
    }
}
