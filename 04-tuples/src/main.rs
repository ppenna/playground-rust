// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

// Suppress some warnings.
#[allow(unused_variables)]

///
/// This tutorial introduces you to the following concepts in Rust:
///  - Tuples
///
fn main() {
    // A tuple is a collection of values of different types.
    let tuple: (usize, &str) = (5, "cat");

    // Values can be extracted from a tuple with indexing.
    let number: usize = tuple.0;
    let string: &str = tuple.1;

    // Tuples can be deconstructed to create bindings.
    let (number, string): (usize, &str) = tuple;
}
