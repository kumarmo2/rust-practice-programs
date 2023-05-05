#![allow(dead_code, unused_variables)]

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;
use std::{cell::RefCell, marker::PhantomData, ops::Deref, rc::Rc};
struct Node<T> {
    val: T,
    next: Pointer<T>,
    prev: Pointer<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Self {
            val,
            next: None,
            prev: None,
        }
    }
}

struct Ends<T> {
    start: Rc<RefCell<Node<T>>>,
    end: Rc<RefCell<Node<T>>>,
}

pub struct DoublyLinkedList<T> {
    ends: Option<Ends<T>>,
}

pub struct Iter<'a, T> {
    current: Pointer<T>,
    lifetime: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.clone().map(|node| {
            self.current = node.clone().deref().borrow().next.clone();
            unsafe { &(*node.as_ptr()).val }
        })
    }
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self { ends: None }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            current: self.ends.as_ref().map(|ends| ends.start.clone()),
            lifetime: PhantomData::default(),
        }
    }

    pub fn add_last(&mut self, val: T) {
        let node = Rc::new(RefCell::new(Node::new(val)));
        if self.ends.is_none() {
            self.ends = Some(Ends {
                start: node.clone(),
                end: node.clone(),
            });
            return;
        }

        let curr_end = self.ends.as_mut().unwrap().end.clone();
        curr_end.borrow_mut().next = Some(node.clone());
        node.borrow_mut().prev = Some(curr_end);
        self.ends.as_mut().unwrap().end = node;
    }
}

#[cfg(test)]
mod tests {
    use crate::DoublyLinkedList;

    #[test]
    fn empty_iter_works() {
        let dll: DoublyLinkedList<i32> = DoublyLinkedList::new();

        let mut iterator = dll.iter();
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn iter_works() {
        let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        dll.add_last(5);
        dll.add_last(6);
        dll.add_last(7);

        let mut iterator = dll.iter();
        assert_eq!(5, *iterator.next().unwrap());
        assert_eq!(6, *iterator.next().unwrap());
        assert_eq!(7, *iterator.next().unwrap());
        assert_eq!(None, iterator.next());
    }
}
