#![allow(dead_code, unused_mut, unused_variables)]
use std::ops::{Deref, DerefMut};

#[derive(Debug)]
pub struct Node<T> {
    pub val: T,
    pub next: Option<Box<Node<T>>>,
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

pub struct NodeIterator<'a, T> {
    head: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(head) = self.head else {
            return None;
        };
        let to_return = head.deref();
        if head.next.is_some() {
            let x: Option<&'a Node<T>> = head.next.as_deref();
            self.head = x;
        } else {
            self.head = None;
        }
        Some(to_return)
    }
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }

    pub fn iter<'a>(&'a self) -> NodeIterator<'a, T> {
        NodeIterator { head: Some(self) }
    }

    pub fn add(&mut self, val: T) {
        let mut next = self.next.as_mut();
        let mut curr = self;

        loop {
            match &curr.next {
                Some(_) => {
                    curr = curr.next.as_mut().unwrap();
                }
                None => {
                    curr.next = Some(Box::new(Node::new(val)));
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let node = Node::new(5);
        assert_eq!(*node.deref(), 5);
        assert_eq!(5, *node.iter().next().unwrap());
    }

    #[test]
    fn adding_two_works() {
        let mut head = Node::new(5);
        head.add(6);

        let mut iterator = head.iter();
        assert_eq!(5, *iterator.next().unwrap());
        assert_eq!(6, *iterator.next().unwrap());
        assert_eq!(None, iterator.next());
    }
}
