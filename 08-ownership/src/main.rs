// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

///
/// This tutorial introduces you to the following concepts in Rust:
///  - References
///  - Ownership
///  - Borrowing
///
fn main() {
    immutable_reference();
    mutable_reference();
    ownerwship();
    immutable_borrow();
    mutable_borrow();

    // TODO: Activity 2
    // - Write a function that takes an array of numbers as parameter and
    // locates the largest element in that array.

    // TODO: Activity 3
    //  - Write a function that takes an array of numbers as a parameter and
    //  reverse the order of those elements in that array.

    // TODO: Activity 4
    // - Write a function that sorts an array of numbers as parameter and sorts
    // that array in-place, using any method that you want.
}

//==============================================================================
// Immutable References
//==============================================================================

///
/// **Immutable Reference**
///
/// - It is a reference to a value.
/// - It allows you to only read the referenced value.
/// - You can have multiple immutable references to the same.
///
fn immutable_reference() {
    let x: usize = 2;
    let r: &usize = &x;
    println!("{} == {}", x, *r);
}

//==============================================================================
// Mutable References
//==============================================================================

///
/// **Mutable Reference**
///
/// - It is a reference to a value.
/// - It allows you to read and write the referenced value.
/// - You can have only one mutable reference to the same value.
///
fn mutable_reference() {
    let mut x: usize = 2;
    let r: &mut usize = &mut x;
    *r -= 2;
    println!("{}", *r);
}
//==============================================================================
// Ownership
//==============================================================================

///
/// **Ownership**
///
/// - It is a set of rules that governs how resources are managed.
/// - Each value has an owner.
/// - There can only be one owner at a time.
/// - When the owner goes out of scope, the value is dropped.
///
fn ownerwship() {
    let mut x: usize = 1;
    let r1: &mut usize = &mut x;

    // TODO: Activity 1
    // - Uncomment the print statement bellow and observe the error.
    // - Explain why can't x be used?
    // println!("{}", x);

    *r1 += 1;

    println!("{}", r1);
}

//==============================================================================
// Immutable Borrow
//==============================================================================

///
/// **Immutable Borrow**
///
/// - It is a reference to a value.
/// - It allows you to only read the referenced value.
/// - You can have multiple immutable references to the same.
///
fn immutable_borrow() {
    let hello: &str = "Hello";

    // Immutable borrow.
    print(&hello);
}

// Prints a string.
fn print(string: &str) {
    println!("{}", string);
}

//==============================================================================
// Mutable Borrow
//==============================================================================

///
/// **Mutable Borrow**
///
/// - It is a reference to a value.
/// - It allows you to read and write the referenced value.
/// - You can have only one mutable reference to the same value.
///
fn mutable_borrow() {
    let mut number: usize = 1;

    // Mutable borrow.
    increment(&mut number);

    println!("number: {}", number)
}

/// Increments a number.
fn increment(number: &mut usize) {
    *number += 1;
}
