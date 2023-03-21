use std::fmt::Debug;
use std::{borrow::Borrow, cell::RefCell, thread};

#[derive(Debug)]
struct MyBox<T>(T)
where
    T: Debug;

impl<T> Drop for MyBox<T>
where
    T: Debug,
{
    fn drop(&mut self) {
        println!("dropping myBox: {:?}", self.0);
    }
}

thread_local! {
    static num: RefCell<i32> = RefCell::new(2);
    static my_box: RefCell<MyBox<i32>> = RefCell::new(MyBox(10));
}

const box1: MyBox<i32> = MyBox(0);
static box2: MyBox<i32> = MyBox(100);
static c: i32 = 100;

fn main() {
    println!("Hello, world!");

    // let handles: Vec<_> = (0..10)
    // .map(|counter| {
    // thread::spawn(move || {
    // num.with(|cell| {
    // let new_val;
    // new_val = *cell.borrow() + 100;
    // *cell.borrow_mut() = new_val;
    // println!("newVal: {}", *cell.borrow());
    // })
    // })
    // })
    // .collect();

    // for handle in handles {
    // handle.join().unwrap();
    // }

    let handles: Vec<_> = (0..10)
        .map(|counter| {
            thread::spawn(move || {
                my_box.with(|cell| {
                    let new_val = cell.borrow().0 + 9;
                    cell.borrow_mut().0 = new_val;
                    println!("from counter: {}, cell: {:?}", counter, cell);
                })
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let mut y = box1;
    let z = box1;

    y.0 = 99;

    println!("y: {:?}", y);
    println!("z: {:?}", z); // NOTE: this will print 0 only because box1 is inlined
                            // where y and z are initialized because of const.

    // let a = box2; // This will not even work unlike const as for static variables, we cannot
    // move out of the variable

    // let a = &mut c;
    // let b = &c;

    let handles: Vec<_> = (0..5)
        .map(|_| {
            thread::spawn(|| {
                println!("c: {}", c);
            })
        })
        .collect();
}
