/*
    Generics in Rust

    "Generic" means across different types!
    Most commonly, two things in Rust are generic: structs and functions.
*/

use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Formatter;

/*
    Generic Structs

    Vec<T> is already an example

    So the <> are how we indicate generics in Rust
    If you're familiar with C++, these are similar to templates.
*/

pub struct SortedVector<T> {
    pub sv: Vec<T>,
    pub is_sorted: bool,
    pub length: usize,
}
// Just like with other (non-generic) types, we can define
// methods, etc. on our type. BUT we have to always
// remember the <T> at the beginning to indicate that this
// type is generic (works for an arbitrary type T).
impl<T> SortedVector<T> {
    pub fn new(raw_data: Vec<T>) -> SortedVector<T> {
        // raw_data.sort();
        let length = raw_data.len();
        SortedVector { sv: raw_data, is_sorted: false, length }

        // Don't worry about the .sort() for now --
        // We will get back to this example later
        // The .sort() doesn't quite work because T could
        // be anything (not necessarily comparable for sorting)
        // We'll see how to get around that.
    }
}

/*
    INTERNAL DETAILS: what does this <T> really mean?

    - Zero-cost abstraction

      In compiled code, I get a different SortedVector implementation
      for every type T for which SortedVector<T> ever occurs!

      In dynamic languages (e.g. Pythong), using a general / abstract type
      costs at runtime because the function has to be able to deal with different
      types.
      In Rust, I get exactly the same performance as if I had implemented

      SortedVectorUsize
      SortedVectorBool
      SortedVectorString

      and so on.

    - C++ templates

      https://codegolf.stackexchange.com/questions/1956/generate-the
      -longest-error-message-in-c
      *Cries in C++*
*/

// Syntax for an impl block
// The impl<T> means there is a separate compiled code for every different T.
impl<T> SortedVector<T> {
    /* What should we implement for SortedVector? */
}

// Can we generalize our AddressBook example?

// Capital letters for types
// D: address book data
// F1, F2: address book fields
pub struct AddressBookGen<F1, F2, D> {
    pub by_field1: HashMap<F1, D>,
    pub by_field2: HashMap<F2, Vec<D>>,
}

// Then different users can use AddressBookGen for different
// kinds of addresses, e.g. a book of people's phones, a book of
// physical addresses, and so on.

/*
    Generic Functions

    Angle brackets go after the name of the function
    The types are then used for arguments or return type
*/

pub fn get_first<T>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        None
    } else {
        Some(&list[0])
    }
}

/*
    Generic Trait Bounds

    What if you want to write a generic function, but it requries a certain kind
    of type, it doesn't work for ALL types?

    Trait bounds to the rescue! Use T: Trait
    to indicate T should be a type which implements Trait.
*/

// Doesn't work
// pub fn print_vec<T>(v: &[T]) {
// Works
pub fn print_vec<T: Debug>(v: &[T]) {
    for ele in v.iter() {
        println!("{:?}", ele);
    }
}

// Similarly if we want to directly implement Debug printing
// for a container type, like our SortedVector,
// we can't implement it for ALL SortedVector, we have to assume
// that T satisfies a certain trait first.
impl<T: Debug> Debug for SortedVector<T> {
    fn fmt(
        &self,
        f: &mut Formatter,
    ) -> ::std::result::Result<(), ::std::fmt::Error> {
        self.sv.fmt(f)
    }
}

// Either the trait or the type must be defined in this crate.
// Avoids clashes in implementations.

impl<P: Ord> Into<Vec<P>> for SortedVector<P> {
    fn into(self) -> Vec<P> {
        unimplemented!();
    }
}

/*
    Iterators!
    Iterators in Rust are powerful, and encapsulated by the
    Iterator **trait*, not a type.

    Q: why is Iterator a trait and not a type?
*/

// For iterators we use "associate types."
pub trait Iterator {
    type Item; // Associated type!
    fn next(&mut self) -> Option<Self::Item>;
}

// See how it's used in collect! Basically magic!
// https://doc.rust-lang.org/src/core/iter/iterator.rs.html#1414
// Collect requires FromIterator:
// https://doc.rust-lang.org/std/vec/struct.Vec.html#impl-FromIterator%3CT%3E

// Traits can be generic too!
trait Graph<N, E> {
    fn has_edge(&self, foo: &N, bar: &N) -> bool;
    // ...
}
