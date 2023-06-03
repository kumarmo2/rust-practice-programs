#![allow(dead_code, unused_imports)]

use std::ops::{Deref, DerefMut};

#[derive(Debug)]
struct Node<T> {
    pub(crate) val: T,
    pub(crate) left: Option<Box<Node<T>>>,
    pub(crate) right: Option<Box<Node<T>>>,
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

impl<T> Node<T> {
    pub(crate) fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct Bst<T>
where
    T: Ord,
{
    pub(crate) head: Option<Box<Node<T>>>,
}

pub struct InorderIterator<'tree, T>
where
    T: Ord,
{
    node: Option<&'tree Box<Node<T>>>,
    stack: Vec<&'tree Box<Node<T>>>,
}

pub struct PreorderIterator<'tree, T>
where
    T: Ord,
{
    node: Option<&'tree Box<Node<T>>>,
    stack: Vec<&'tree Box<Node<T>>>,
}

impl<'tree, T> Iterator for PreorderIterator<'tree, T>
where
    T: Ord,
{
    type Item = &'tree T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_none() && self.stack.is_empty() {
            return None;
        }
        let node = match self.node {
            Some(x) => x,
            None => self.stack.pop().unwrap(),
        };

        // loop {
        // let Some(node) = self.node else {
        // break;
        // };
        if let Some(ref right_child) = node.as_ref().right {
            self.stack.push(right_child);
        }
        let result = node.as_ref().deref();
        self.node = node.as_ref().left.as_ref();
        return Some(result);
        // }
        // let Some(node) =  self.stack.pop() else {
        // return None;
        // };
        // if let Some(ref right_child) = node.as_ref().right {
        // self.stack.push(right_child);
        // }
        // let result = node.as_ref().deref();
        // self.node = node.as_ref().left.as_ref();
        // return Some(result);
    }
}

impl<'tree, T> Iterator for InorderIterator<'tree, T>
where
    T: Ord,
{
    type Item = &'tree T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.node.is_none() && self.stack.is_empty() {
            return None;
        }

        loop {
            let Some(node) = self.node else {
                break;
            };

            self.node = node.left.as_ref();
            self.stack.push(node);
        }
        if self.stack.is_empty() {
            return None;
        }

        let to_return = self.stack.pop().unwrap();
        self.node = to_return.right.as_ref();
        return Some(to_return.deref());
    }
}

impl<T> Bst<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn add(&mut self, val: T) {
        if let None = self.head {
            self.head = Some(Box::new(Node::new(val)));
            return;
        }

        let mut curr = &mut self.head;

        loop {
            let Some(node) = curr else {
                break;
            };

            if val <= node.val && node.left.is_none() {
                node.left = Some(Box::new(Node::new(val)));
                break;
            }
            if val > node.val && node.right.is_none() {
                node.right = Some(Box::new(Node::new(val)));
                break;
            }
            if val <= node.val {
                curr = &mut node.left;
            } else {
                curr = &mut node.right;
            }
        }
    }

    pub fn inorder_iter<'tree>(&'tree self) -> InorderIterator<'tree, T> {
        InorderIterator {
            node: self.head.as_ref(),
            stack: Vec::new(),
        }
    }
    pub fn preorder_iter<'tree>(&'tree self) -> PreorderIterator<'tree, T> {
        PreorderIterator {
            node: self.head.as_ref(),
            stack: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preorder_it_works_for_empty_bst() {
        let bst: Bst<i32> = Bst::new();

        let mut iter = bst.preorder_iter();
        assert_eq!(None, iter.next());
    }

    #[test]
    fn preorder_it_works_for_one_item_bst() {
        let mut bst: Bst<i32> = Bst::new();
        bst.add(2);

        let mut iter = bst.preorder_iter();
        assert_eq!(2, *iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn preorder_it_works_for_two_items_bst() {
        let mut bst: Bst<i32> = Bst::new();
        bst.add(2);
        bst.add(1);

        let mut iter = bst.preorder_iter();
        assert_eq!(2, *iter.next().unwrap());
        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(None, iter.next());

        let mut bst: Bst<i32> = Bst::new();
        bst.add(1);
        bst.add(2);

        let mut iter = bst.preorder_iter();
        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn preorder_it_works_for_multiple_item_bst() {
        let mut bst: Bst<i32> = Bst::new();
        bst.add(5);
        bst.add(10);
        bst.add(3);
        bst.add(4);
        bst.add(1);
        bst.add(2);

        let mut iter = bst.preorder_iter();
        assert_eq!(5, *iter.next().unwrap());
        assert_eq!(3, *iter.next().unwrap());
        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
        assert_eq!(4, *iter.next().unwrap());
        assert_eq!(10, *iter.next().unwrap());
        assert_eq!(None, iter.next());
    }
    #[test]
    fn it_works_for_empty_bst() {
        let bst: Bst<i32> = Bst::new();

        let mut iter = bst.inorder_iter();
        assert_eq!(None, iter.next());
    }

    #[test]
    fn it_works_for_one_item_bst() {
        let mut bst: Bst<i32> = Bst::new();
        bst.add(2);

        let mut iter = bst.inorder_iter();
        assert_eq!(2, *iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn it_works_for_two_items_bst() {
        let mut bst: Bst<i32> = Bst::new();
        bst.add(2);
        bst.add(1);

        let mut iter = bst.inorder_iter();
        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
        assert_eq!(None, iter.next());

        let mut bst: Bst<i32> = Bst::new();
        bst.add(1);
        bst.add(2);

        let mut iter = bst.inorder_iter();
        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn it_works_for_multiple_item_bst() {
        let mut bst: Bst<i32> = Bst::new();
        bst.add(5);
        bst.add(10);
        bst.add(3);
        bst.add(4);
        bst.add(1);
        bst.add(2);

        let mut iter = bst.inorder_iter();
        assert_eq!(1, *iter.next().unwrap());
        assert_eq!(2, *iter.next().unwrap());
        assert_eq!(3, *iter.next().unwrap());
        assert_eq!(4, *iter.next().unwrap());
        assert_eq!(5, *iter.next().unwrap());
        assert_eq!(10, *iter.next().unwrap());
        assert_eq!(None, iter.next());
    }
}
