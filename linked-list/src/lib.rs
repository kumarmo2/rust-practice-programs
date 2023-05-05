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

pub struct LinkedList<T> {
    pub head: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn add(&mut self, val: T) {
        if let None = self.head {
            self.head = Some(Box::new(Node::new(val)));
            return;
        }
        // TODO: remove unwraps.
        let mut curr: &mut Option<Box<Node<T>>> = &mut self.head;
        while curr.as_deref_mut().unwrap().next.is_some() {
            curr = &mut curr.as_deref_mut().unwrap().next;
        }
        curr.as_deref_mut().unwrap().next = Some(Box::new(Node::new(val)));
    }

    pub fn iter<'a>(&'a self) -> NodeIterator<'a, T> {
        NodeIterator {
            head: self.head.as_ref(),
        }
    }

    pub fn iter_mut<'a>(&'a mut self) -> MutNodeIterator<'a, T> {
        MutNodeIterator {
            current: self.head.as_mut(),
        }
    }
}

pub struct NodeIterator<'a, T> {
    head: Option<&'a Box<Node<T>>>,
}
pub struct MutNodeIterator<'a, T> {
    current: Option<&'a mut Box<Node<T>>>,
}

// reference: https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html
impl<'a, T> Iterator for MutNodeIterator<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_mut();
            &mut node.val
        })
    }
}
impl<'a, T> Iterator for NodeIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let head = match self.head {
            Some(head) => head,
            None => return None,
        };
        let to_return = Some(head.deref().deref());

        if head.next.is_some() {
            self.head = head.next.as_ref();
        } else {
            self.head = None;
        }
        to_return
    }
}

impl<T> Node<T> {
    pub fn new(val: T) -> Self {
        Self { val, next: None }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut ll = LinkedList::new();
        ll.add(5);
        ll.add(6);

        let mut iterator = ll.iter();
        assert_eq!(5, *iterator.next().unwrap());
        assert_eq!(6, *iterator.next().unwrap());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn iter_mut_works() {
        let mut ll = LinkedList::new();
        ll.add(5);
        ll.add(13);

        for val in ll.iter_mut() {
            *val = *val + 20;
        }

        let mut iterator = ll.iter();
        assert_eq!(25, *iterator.next().unwrap());
        assert_eq!(33, *iterator.next().unwrap());
        assert_eq!(None, iterator.next());
    }
}
