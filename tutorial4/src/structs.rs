/*
    Named field structs
*/
// As with enums, we often #[derive(...)] to get "free" implementations of
// useful functionality like debug printing, equality checking, cloning, etc.
// Clone, Debug, Eq, PartialEq are examples of "Traits", a lot more on them later!
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Computer {
    ram: i32,
    harddrive: i32,
    cpu_type: String,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct TwoComputers {
    comp1: Computer,
    comp2: Computer,
    len: usize,
}

// Syntax for creating an object:
pub fn make_computer1() -> Computer {
    Computer {
        ram: 4,
        harddrive: 1024,
        cpu_type: "Intel Core i7 7600".to_owned(),
    }
}

#[test]
pub fn test_computer() {
    let mut comp1 = make_computer1();
    let comp2 = make_computer1();
    println!("{:?}", comp1);
    assert_eq!(comp1, comp2);
    assert_eq!(comp1, comp1.clone());
    comp1.ram = 8;
    assert!(comp1 != comp2);
}

pub fn make_computer2(cpu_type: String) -> Computer {
    // Variables already in scope somewhere:
    let ram = 4;
    let harddrive = 1024;
    // ...

    // No need for field: EXPR syntax.
    Computer { ram, harddrive, cpu_type }
}

// Tuple like structs:
pub struct Pair(i32, i32);

pub fn pair() -> Pair {
    Pair(3, 3)
}

pub fn use_pair(p: Pair) {
    println!("{}", p.0);
    println!("{}", p.1);
    // pattern matching
    let Pair(_x, _y) = p;
}

/*
    Some other struct idioms
*/
// This is called a "newtype"
// Mainly used to wrap types.

// Lets you as a programmer keep straight different things that should
// be conceptually different
#[derive(Debug)]
pub struct Area(f64);
#[derive(Debug)]
pub struct Length(f64);
pub fn calculate_area(l1: Length, l2: Length) -> Area {
    Area(l1.0 * l2.0)
}

// Zero overhead abstraction!
// In actual compiled code, Area and Length will be no different from f64.

#[test]
fn test_calculate_area() {
    println!("{:?}", calculate_area(Length(1.0), Length(1.0)));
}

// Using type X = Y; only creates a type synonym or alias, these types are exactly the
// same and interchangeable, if we don't want this, we can create to uniquely different
// types using:
// type Area = f64; // not usually as good
// Advantage: don't have to re-#derive stuff and use the .0 syntax
// Disadvantage: no static typing guarantees to keep types separate.

// Another example: Sorted vector
pub struct SortedVector(Vec<i32>);

// Unit like struct:
pub struct Trivial; // No members...

// In memory, values are stack allocated, just like C++ or C. This avoids heap allocation
// of values (which is what happen in Python, Java, etc).

// How would our struct Computer look in memory?
// No guarantee of ordering of fields, and remember there is padding!

// Use #[repr(C)] for identical C implementation.

/*
    impl syntax: Methods on structs!

    (similar to object oriented programming)

    We have already seen these for e.g. vector:
        v.push(1)
        v.clear()
    Internally the same as other functions, just
    in a different scope and syntactically different
*/
impl SortedVector {
    // Associated function.
    // Basically a "static" function in OO terminology.
    // nothing special about the word "new" just a convention.
    fn new(mut vec: Vec<i32>) -> Self {
        // mut keyword: similar to let mut vec = ... but for
        // owned function argument
        // or -> SortedVector
        vec.sort_unstable();
        SortedVector(vec)
    }

    // For methods, either &self or &mut self

    pub fn print(&self) {
        println!("{:?}", self.0);
    }

    fn push(&mut self, val: i32) {
        // Must explicitly use self to refer to fields in Sorted Vector
        self.0.push(val);
        self.0.sort_unstable();
    }

    // Less common: methods which consume the object

    fn split(self) -> (SortedVector, SortedVector) {
        unimplemented!();
    }
}

pub fn do_sorted_stuff() {
    let mut sv = SortedVector::new(vec![1, 2, 3]);
    // ...
    sv.push(3);

    let (_sv1, _sv2) = sv.split();

    // What happens?
    // sv.push(3);
}

// Functions not defined in any impl block are free functions.

/*
    Interior Mutability
*/

pub struct MutExample {
    field1: usize,
    field2: String,
}
impl MutExample {
    pub fn immut_method(&self) {
        // self.field1 = 0; // not possible
        if self.field1 != self.field2.len() {
            panic!("Invariant violated :(")
        } else if self.field1 == 0 {
            println!("warning: field1 is 0");
        }
    }
    pub fn mut_method(&mut self) {
        self.field1 += 1;
        self.field2.push_str(" words words words");
    }

    pub fn consuming_method(self) {
        // consumes the struct
        println!("Goodbye, {}", self.field1);
        println!("Goodbye, {}", self.field2);
        // **memory deallocated**
        // can be explicit if we want:
        drop(self);
    }

    pub fn associated_fun() {
        println!("I don't have any self parameter");
    }

    // What if we want mutability for some fields but not others?
    // Two main ways:

    // First way:
    fn mutate_field2(field2: &mut String) {
        field2.push_str(" words words words");
    }
    pub fn mutate_object(&mut self) {
        Self::mutate_field2(&mut self.field2);

        // Note that &self is still mutable, so how does this help?
        // Recall: in Rust you either have one mutable reference,
        // or many immutable references.
        // So if you need to have a bunch of immutable references to
        // self.field1, this allows you to still pass an immutable
        // ref to self.field1.
        // Sometimes this solves certain lifetime-related issues.
    }
}

// Second way: "Interior Mutability"
/*
    This gets into some of the more advanced features of Rust

    In general, Rust follows the following philosophy:
    - There is some "default" behavior with respect to mutability, ownership,
      etc.
    - If you want something different, you "opt out" or "opt in" to more
      complex behavior by using one of the type wrappers in the standard
      library

    Some of the most important type wrappers:
    - Arc, Rc: for shared ownership
    - Box: store data on the heap that would normally be on the stack
    - Cell, RefCell: interior mutability
*/
use std::cell::Cell;

#[allow(dead_code)]
#[derive(Debug)]
pub struct MutExampleBetter {
    field1: usize,
    field2: Cell<usize>,
}
impl MutExampleBetter {
    // Modify only field2
    pub fn modify(&self) {
        // self.field1 += 1; // doesn't work
        self.field2.replace(self.field2.get() + 1);
        println!("Self: {:?}", self);
    }
}
