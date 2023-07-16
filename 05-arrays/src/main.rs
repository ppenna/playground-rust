// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

// Suppress some warnings.
#[allow(unused_variables)]

///
/// This tutorial introduces you to the following concepts in Rust:
///  - Arrays
///  - Slices
///
fn main() {
    // An array is a collection of objects of the same type.
    // Arrays have a fixed-size length, known at compile time.
    let array: [i32; 3] = [1, 2, 3];

    // Arrays can be indexed. Indexes start at zero.
    let first: i32 = array[0];
    let second: i32 = array[1];
    let third: i32 = array[2];

    // Arrays can be initialized with the same value.
    let ones: [i32; 4096] = [1; 4096];

    // A slice provides a view of an array.
    // The length of a slice is not known at compile time.
    let slice: &[i32] = &array[..2];

    // TODO: Activity 1
    // - Attempt to access past the end of the array.
    // - Attempt to build and run this example.

    // TODO: Activity 2
    // - Attempt to access past the end of the slice.
    // - Attempt to build and run this example.
}
