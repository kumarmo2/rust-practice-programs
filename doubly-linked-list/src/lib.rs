// #![allow(dead_code, unused_variables)]

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
    front: Rc<RefCell<Node<T>>>,
    back: Rc<RefCell<Node<T>>>,
}

pub struct DoublyLinkedList<T> {
    ends: Option<Ends<T>>,
}

pub struct Iter<'a, T> {
    front: Pointer<T>,
    back: Pointer<T>,
    _marker: PhantomData<&'a T>,
}

pub struct IterMut<'a, T> {
    front: Pointer<T>,
    back: Pointer<T>,
    _marker: PhantomData<&'a T>,
}

fn ptr_eq<T>(a: *mut T, b: *mut T) -> bool {
    a == b
}
impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.clone().map(|node| {
            let back_ptr = node.clone().deref().as_ptr();
            let front_ptr = self.front.clone().unwrap().deref().as_ptr();

            if ptr_eq(front_ptr, back_ptr) {
                self.front = None;
                self.back = None;
            } else {
                self.back = node.clone().deref().borrow().prev.clone();
            }
            unsafe { &mut (*node.as_ptr()).val }
        })
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.front.clone().map(|node| {
            let front_ptr = node.clone().deref().as_ptr();
            let end_ptr = self.back.clone().unwrap().deref().as_ptr();

            if ptr_eq(front_ptr, end_ptr) {
                self.front = None;
                self.back = None;
            } else {
                self.front = node.clone().deref().borrow().next.clone();
            }

            unsafe { &mut (*node.as_ptr()).val }
        })
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.back.clone().map(|node| {
            let back_ptr = node.clone().deref().as_ptr();
            let front_ptr = self.front.clone().unwrap().deref().as_ptr();

            if ptr_eq(front_ptr, back_ptr) {
                self.front = None;
                self.back = None;
            } else {
                self.back = node.clone().deref().borrow().prev.clone();
            }
            unsafe { &(*node.as_ptr()).val }
        })
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.front.clone().map(|node| {
            let front_ptr = node.clone().deref().as_ptr();
            let end_ptr = self.back.clone().unwrap().deref().as_ptr();

            if ptr_eq(front_ptr, end_ptr) {
                self.front = None;
                self.back = None;
            } else {
                self.front = node.clone().deref().borrow().next.clone();
            }
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
            front: self.ends.as_ref().map(|ends| ends.front.clone()),
            back: self.ends.as_ref().map(|ends| ends.back.clone()),
            _marker: PhantomData::default(),
        }
    }
    pub fn iter_mut<'a>(&'a self) -> IterMut<'a, T> {
        IterMut {
            front: self.ends.as_ref().map(|ends| ends.front.clone()),
            back: self.ends.as_ref().map(|ends| ends.back.clone()),
            _marker: PhantomData::default(),
        }
    }

    pub fn add_last(&mut self, val: T) {
        let node = Rc::new(RefCell::new(Node::new(val)));
        if self.ends.is_none() {
            self.ends = Some(Ends {
                front: node.clone(),
                back: node.clone(),
            });
            return;
        }

        let curr_end = self.ends.as_mut().unwrap().back.clone();
        curr_end.borrow_mut().next = Some(node.clone());
        node.borrow_mut().prev = Some(curr_end);
        self.ends.as_mut().unwrap().back = node;
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
    #[test]
    fn mut_iter_works() {
        let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
        dll.add_last(5);
        dll.add_last(6);
        dll.add_last(7);

        for item in dll.iter_mut() {
            *item = *item + 20;
        }

        let mut iterator = dll.iter();
        assert_eq!(25, *iterator.next().unwrap());
        assert_eq!(26, *iterator.next().unwrap());
        assert_eq!(27, *iterator.next().unwrap());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn one_item_double_iter_mut_works() {
        let mut dll = DoublyLinkedList::new();
        dll.add_last(5);

        let mut iter = dll.iter_mut();

        let val = iter.next_back().unwrap();
        *val = *val + 2;

        assert_eq!(None, iter.next());

        let mut iter = dll.iter();
        assert_eq!(7, *iter.next().unwrap());
        assert_eq!(None, iter.next());
    }
    #[test]
    fn multiple_items_double_iter_mut_works() {
        let mut dll = DoublyLinkedList::new();
        dll.add_last(5);
        dll.add_last(13);
        dll.add_last(19);

        let mut iter = dll.iter_mut();

        let val = iter.next_back().unwrap();
        *val = *val + 2;

        let val = iter.next().unwrap();
        *val = *val + 9;

        let val = iter.next_back().unwrap();
        *val = *val + 4;

        assert_eq!(None, iter.next());

        let mut iter = dll.iter();
        assert_eq!(14, *iter.next().unwrap());
        assert_eq!(21, *iter.next_back().unwrap());
        assert_eq!(17, *iter.next_back().unwrap());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn one_item_double_iter_works() {
        let mut dll = DoublyLinkedList::new();
        dll.add_last(5);
        let mut iterator = dll.iter();
        assert_eq!(5, *iterator.next_back().unwrap());
        assert_eq!(None, iterator.next());
    }
    #[test]
    fn two_item_double_iter_works() {
        let mut dll = DoublyLinkedList::new();
        dll.add_last(5);
        dll.add_last(6);
        let mut iterator = dll.iter();
        assert_eq!(6, *iterator.next_back().unwrap());
        assert_eq!(5, *iterator.next().unwrap());
        assert_eq!(None, iterator.next_back());
    }

    #[test]
    fn double_iter_works() {
        let mut dll = DoublyLinkedList::new();
        dll.add_last(1);
        dll.add_last(2);
        dll.add_last(3);
        dll.add_last(4);
        dll.add_last(5);
        dll.add_last(6);

        let mut iter = dll.iter();
        assert_eq!(Some(&1), iter.next());
        assert_eq!(Some(&6), iter.next_back());
        assert_eq!(Some(&5), iter.next_back());
        assert_eq!(Some(&2), iter.next());
        assert_eq!(Some(&3), iter.next());
        assert_eq!(Some(&4), iter.next());
        assert_eq!(None, iter.next());
        assert_eq!(None, iter.next_back());
    }
}
