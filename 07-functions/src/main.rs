// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

///
/// This tutorial introduces you to the following concepts in Rust:
///  - Functions
///  - Nested functions
///  - Closures
///
fn main() {
    let a: isize = -23;
    let b: isize = 7;

    print(a);
    print(b);

    let result: isize = add(a, b);
    println!("{} + {} = {}", a, b, result);

    let result: usize = abs(a);
    println!("|{}| = {}", a, result);

    /// This a nested function.
    fn greater_than(x: isize, y: isize) -> bool {
        x > y
    }
    println!("{} > {} ? ({})", a, b, greater_than(a, b));

    // This is a closure.
    let equal = |x: isize, y: isize| -> bool { x == y };
    println!("{} == {} ? ({})", a, b, equal(a, b));

    // TODO: Activity 1
    //  - Write a function that computes the factorial of a number.

    // TODO: Activity 2
    // - Write a function that computes the nth term of the Fibonacci sequence.
}

/// This is a function takes a value and does not return anything.
/// The return type could be omitted.
pub fn print(val: isize) -> () {
    println!("val: {}", val);
}

//// This function takes two values as parameter, and returns a value.
/// The return value is the last expression listed in the function.
pub fn add(a: isize, b: isize) -> isize {
    a + b
}

/// This function takes one value as parameter and returns a value.
/// The `return` keyword may be omitted only for the last expression
/// listed in t the function.
pub fn abs(x: isize) -> usize {
    if x < 0 {
        return -x as usize;
    }

    x as usize
}
