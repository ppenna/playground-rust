use std::collections::HashMap;

/*
    Heap Allocation: Box

    Why do I need to allocate something on the heap?

    Normally when you have variable like

    let x = 3;
    let v = Vec::new();

    it's allocated on the stack
    Internally the vector memory is on the heap, but the vector object is basically
    a pointer which is on the stack.

    This only works because everything has a known Size at compile time

    Sized trait: https://doc.rust-lang.org/std/marker/trait.Sized.html

    - Box dyn

    - Useful for recursively defined data types

*/

#[test]
fn example_box_dyn() {
    use std::fmt::Debug;

    let mut v: Vec<Box<dyn Fn() -> usize>> = Vec::new();
    v.push(Box::new(|| 3));
    v.push(Box::new(|| 4));
    for func in v {
        println!("{}", func());
    }

    // Note that || 3 and || 4 are completely different tyeps so
    // they don't both have a size known at compile time,
    // but once converted to a pointer to heap memory, Box<dyn,
    // that thing has a known size (the pointer size for whatever
    // OS you're on, probably 64 bit)

    // You can also do box dyn for other traits.
    let mut v: Vec<Box<dyn Debug>> = Vec::new();
    v.push(Box::new(3));
    v.push(Box::new("hello"));
    v.push(Box::new("world".to_string()));

    for x in v {
        println!("{:?}", x);
    }

    // Note: can only do what the trait offers once it's a dyn trait object
    // For example can't do:
    // println!("{}", v[0] + v[0]);

    // This is because of the dyn Trait
    // Box is fine, it's interchangeable with original type
    let x = Box::new(3);
    let y = Box::new(4);
    println!("{}", *x + *y);
    let z: usize = *x;
    println!("{}", z);

    // Interesting restrictions on which traits can be converted to a Box dyn
    // Clone can't for example
    // Has to do with how dynamic trait objects are implemented
    // (May change in a future version of Rust)
    // let mut v: Vec<Box<dyn Clone>> = Vec::new();
    // v.push(Box::new(3));
    // v.push(Box::new("hello"));
    // v.push(Box::new("world".to_string()));
    // v.push(v[0].clone());

    // assert!(false);
}

/*
    The one place that Box shows up a lot:
    recursive data types
*/

pub enum FuncList<T> {
    Nil,                       // empty list
    Cons(T, Box<FuncList<T>>), // head: T followed by a tail FuncList<T>
}

// Idea: without the Box, we would need:
// size_of(FuncList<T>) >= size_of(T) + size_of(FuncList<T>)
// ^ this is impossible
// With the Box, Box has a constant size (pointer)
// size_of(FuncList<T>) = size_of(T) + size_of(pointer)
// (roughtly size of T plus a byte)

/*
    Shared ownership: Rc and Arc

    See id_manager

    Arc is the same as Rc except it's thread-safe
    (You would use Arc for concurrent programming with shared ownership)
*/

/*
    Interior Mutability: Cell and RefCell

    Recall Rust rule:
    - everything can only have ONE mutable reference to it,
      OR any number of immutable references.

    Sometimes you need to get around this and have
    multiple references to something that can all mutate it.

    There's nothing inherently wrong with that, but you'd expect it
    to be less efficient in a concurrent setting (each reference
    when it's modifying the object has to check that no one else
    is modifying it)

    Cell: statically checked for Copy types

    RefCell: dynamically checked at runtime and has overhead

    Upshot: if you want, you can have an immutable object
    with mutable fields.
*/

use std::cell::Cell;

// Suppose we have a Cache
// and we also internally want to track cache hits and cache misses
// transparently, without exposing that to the user
// -> only do this debug mode, print it out to a log
pub struct Cache {
    cache: HashMap<usize, usize>,
    hits: Cell<usize>,
    misses: Cell<usize>,
}
impl Cache {
    pub fn save(&mut self, x: usize, y: usize) {
        self.cache.insert(x, y);
    }
    pub fn query(&self, x: usize) -> Option<usize> {
        match self.cache.get(&x) {
            Some(&x) => {
                self.hits.set(self.hits.get() + 1);
                Some(x)
            }
            None => {
                self.misses.set(self.misses.get() + 1);
                None
            }
        }
    }
}

/*
    RefCell:

    Cell only works above for simple Copy types, like usize.
    It avoids runtime overhead by copying memory in and out of the cell.

    In general to do this though for an arbitrary type requries runtime
    checking of the borrow rules, and is done with RefCell.

    To get around both shared ownership AND mutability rules, you will
    often see code with

    Rc<RefCell<T>>.
*/

use std::cell::RefCell;
use std::rc::Rc;

pub struct RefCellExample {
    previous: Rc<RefCell<Vec<usize>>>,
    next: Rc<RefCell<Vec<usize>>>,
}
impl RefCellExample {
    pub fn modify_with_immut_self(&self) {
        self.previous.borrow_mut().push(3);
        self.next.borrow_mut().push(4);
    }
}
