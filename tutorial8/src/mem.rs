use std::fmt::Debug;

/*
    The Drop Trait
*/

#[derive(Debug)]
pub struct Person {
    pub name: String,
    pub age: u8,
}

// What happens internally with the above code when a Person goes out of scope?

// There is an explicit drop statement which is occasionally useful:
#[test]
fn test_drop_person() {
    // define a person
    let me = Person { name: "Caleb".to_string(), age: 26 };
    println!("{:?}", me);

    drop(me);

    // println!("{:?}", me); // Compile time error
}

// We can manually choose what happens when Drop is called if we want
// to customize our memory management.
// This is done via the Drop trait.
// This is like implementing a Destructor in C++

// (Aside: from C++ programming: rule of 3
// https://en.wikipedia.org/wiki/Rule_of_three_(C%2B%2B_programming)
// In Rust if you implement Drop manually, you might also consider
// implementing Clone manually; however since there is no unsafety
// in Rust, we don't need to worry about the rule of 3 in general.)

// For example: let's leak memory!

impl Drop for Person {
    fn drop(&mut self) {
        // do nothing!
        // Leaks memory. (Technically leaking memory is allowed in Rust,
        // it is not prevented by the compiler in all edge cases,
        // though Rust still makes us be very explicit if that is what
        // we want to do.)
    }
}

// Perhaps a more practical example:
// Here is a weird struct for making a dynamically allocated string that lives
// permanently
// This can also be done without unsafe pointers,
// e.g. with the function Box::leak, the below is just for illustration

use std::ops::Deref;

pub struct PermanentString {
    contents: Box<String>,
    contents_ptr: *const str,
}
impl PermanentString {
    pub fn new(s: &str) -> Self {
        let contents = Box::new(s.to_string());
        let contents_ptr: *const str = contents.as_ref().deref();
        Self { contents, contents_ptr }
    }
    pub fn get_temporary_reference(&self) -> &str {
        self.contents.as_ref()
    }
    pub fn get_permanent_reference(&self) -> &'static str {
        unsafe { self.contents_ptr.as_ref().unwrap() }
    }
}

impl Drop for PermanentString {
    fn drop(&mut self) {
        // we do NOT want to drop self.contents here,
        // because we want permanent references to remain valid.

        // We can imagine more compelx logic, for example,
        // store a bool of whether a permanent reference
        // was given out, and if not, drop self.contents here.
    }
}

/*
    Size of operator
*/

use std::mem::size_of;

pub fn sizeof_operator() {
    // let t = (3, 3i8);
    let _size = size_of::<(i32, i8)>();
}

/*
    mem::replace
    https://doc.rust-lang.org/std/mem/fn.replace.html

    This comes in handy to swap values efficiently,
    for example swap two vector variables.

    Or if you have e.g. a v: Vec<Vec<T>>
    you can replace v[3] with a different vector
    using an O(1) operation.

    Example from the reading:
    https://rust-unofficial.github.io/too-many-lists/first-final.html
*/
