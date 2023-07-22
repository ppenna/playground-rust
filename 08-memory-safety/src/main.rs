// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

///
/// This tutorial introduces you to the following concepts in Rust:
///  - References
///  - Ownership
///  - Borrowing
///  - Lifetimes
///  - Unsafe
///
fn main() {
    immutable_reference();
    mutable_reference();
    ownerwship();
    immutable_borrow();
    mutable_borrow();
    lifetime();
    unsafe_();

    // TODO: Activity 4
    // - Write a function that takes an array of numbers as parameter and
    // locates the largest element in that array.

    // TODO: Activity 5
    //  - Write a function that takes an array of numbers as a parameter and
    //  reverse the order of those elements in that array.

    // TODO: Activity 6
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

//==============================================================================
// Lifetimes
//==============================================================================

///
/// **Lifetimes **
///
/// - Annotations that detail how long a reference is valid.
/// - Prevents dangling references
/// - Implicitly introduced by the compiler.
/// - Are coerced by the compiler.
///
fn lifetime() {
    let string1: &str = "Operating";
    let string2: &str = "Systems";

    // Implicit lifetime annotation.
    let longest_string: &str = longest(string1, string2);

    println!("Longest string: {}", longest_string);

    // TODO: Activity 1
    // - Remove lifetime annotations from the function and observe the error.
    // - Explain why can't x be used?
}

/// Returns the longest string between two strings.
fn longest<'a>(string1: &'a str, string2: &'a str) -> &'a str {
    // Note that all references have the same lifetime 'a.
    if string1.len() > string2.len() {
        string1
    } else {
        string2
    }
}

//==============================================================================
// Unsafe
//==============================================================================

/// Global variable.
static mut GLOBAL_VARIABLE: usize = 0;

///
/// **Unsafe**
///
/// - Allows you to bypass the borrow checker.
/// - Enables you to dereference raw pointers.
/// - Enables you to call unsafe functions.
/// - Enables you to access or modify mutable static variables.
/// - Enables you to implement unsafe traits.
/// - Enables you to access fields of unions.
///
fn unsafe_() {
    dereference_raw_pointer();
    unsafe { unsafe_function() };
}

/// Unsafe function that modifies a global variable.
unsafe fn unsafe_function() {
    GLOBAL_VARIABLE += 1;
    println!("GLOBAL_VARIABLE: {}", GLOBAL_VARIABLE);
}

/// Dereferences a raw pointer.
fn dereference_raw_pointer() {
    let mut x: usize = 1;
    let r: *mut usize = &mut x;

    // The following code is unsafe because we dereference a raw pointer.
    unsafe {
        *r += 1;
    }

    println!("x: {}", x);
}
