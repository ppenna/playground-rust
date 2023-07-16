// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

// Suppress some warnings.
#![allow(dead_code, unused_variables)]

///
/// This tutorial introduces you to the following concepts in Rust:
///  - Primitive Data Types
///  - Casting
///  - Aliasing
///  - Arithmetic Operations
///  - Bitwise Operations
///  - Logical Operations
///  - Comparison Operations
///
fn main() {
    // Signed integers.
    let int8: i8 = i8::MAX;
    let int16: i16 = i16::MAX;
    let int32: i32 = i32::MAX;
    let int64: i64 = i64::MAX;
    let int128: i128 = i128::MAX;
    let int: isize = isize::MAX;

    // Unsigned integers.
    let uint8: u8 = u8::MAX;
    let uint16: u16 = u16::MAX;
    let uint32: u32 = u32::MAX;
    let uint64: u64 = u64::MAX;
    let uint128: u128 = u128::MAX;
    let uint: usize = usize::MAX;

    // Floating point integers.
    let float32: f32 = 3.14;
    let float64: f64 = 3.14;

    // Boolean.
    let boolean: bool = true;
    let boolean: bool = false;

    // Character.
    let character: char = 'x';

    // String. Strings are UTF-8 encoded and immutable.
    let string: &str = "Hello, folks!";

    // Empty.
    let empty: () = ();

    // Casting.
    let uint: usize = uint8 as usize;

    // Aliasing. Type aliases don't provide extra type safety.
    // Custom types must have UpperCamelCase names.
    type Kilogram = f32;

    // TODO: Activity 1
    // - Attempt to mutate a string.
    // - Attempt to build and run this example.

    // TODO: Activity 2
    // - Cast a u128 to a usize.
    // - Print the resulting value.
    // - Compare the resulting value with the max values for u128 and usize.
    // - Attempt to build and run this example.

    // TODO: Activity 3
    // - Cast a float to a char.
    // - Attempt to build and run this example.

    // TODO: Activity 4
    // - Cast a f64 to a 'Kilogram'.
    // - Attempt to build and run this example.

    // TODO: Activity 5
    // - Check out the arithmetic_operations() function.
    // - Attempt to build and run it:
    //  'cargo test --bin primitive-types -- arithmetic_operations --nocapture'

    // TODO: Activity 6
    // - Check out the bitwise_operations() function.
    // - Attempt to build and run it:
    //  'cargo test --bin primitive-types -- bitwise_operations --nocapture'

    // TODO: Activity 7
    // - Check out the logical_operations() function.
    // - Attempt to build and run it:
    //  'cargo test --bin primitive-types -- logical_operations --nocapture'

    // TODO: Activity 8
    // - Check out the comparison_operations() function.
    // - Attempt to build and run it:
    //  'cargo test --bin primitive-types -- comparison_operations --nocapture'
}

//==============================================================================
// Arithmetic Operations
//==============================================================================

#[test]
fn arithmetic_operations() {
    let a: usize = 23;
    let b: usize = 7;

    // Addition.
    let result: usize = a + b;
    println!("{:>24} {} + {} = {}", "Addition:", a, b, result);

    // Subtraction.
    let result: usize = a - b;
    println!("{:>24} {} - {} = {}", "Subtraction:", a, b, result);

    // Multiplication.
    let result: usize = a * b;
    println!("{:>24} {} * {} = {}", "Multiplication:", a, b, result);

    // Division.
    let result: usize = a / b;
    println!("{:>24} {} / {} = {}", "Division:", a, b, result);

    // Remainder of a Division.
    let result: usize = a % b;
    println!(
        "{:>24} {} % {} = {}",
        "Remainder of a Division:", a, b, result
    );
}

//==============================================================================
// Bitwise Operations
//==============================================================================

#[test]
fn bitwise_operations() {
    let a: usize = 0b1010;
    let b: usize = 0b1100;

    // AND.
    let result: usize = a & b;
    println!("{:>12} {:04b} = {:04b} & {:04b}", "AND:", a, b, result);

    // OR.
    let result: usize = a | b;
    println!("{:>12} {:04b} = {:04b} | {:04b}", "OR:", a, b, result);

    // XOR.
    let result: usize = a ^ b;
    println!("{:>12} {:04b} = {:04b} ^ {:04b}", "XOR:", a, b, result);

    // Sift left.
    let result: usize = a << 1;
    println!("{:>12} {:04b} = {:04b} << 1", "Shift Left:", a, result);

    // Sift right.
    let result: usize = a >> 1;
    println!("{:>12} {:04b} = {:04b} >> 1", "Shift Right:", a, result);
}

//==============================================================================
// Logical Operations
//==============================================================================

#[test]
fn logical_operations() {
    let a: bool = true;
    let b: bool = false;

    // AND.
    let result: bool = a && b;
    println!("{:>4} {} = {} && {}", "AND:", result, a, b);

    // OR.
    let result: bool = a || b;
    println!("{:>4} {}  = {} || {}", "OR:", result, a, b);

    // NOT.
    let result: bool = !a;
    println!("{:>4} {} = !{}", "NOT:", result, a);
}

//==============================================================================
// Comparison Operations
//==============================================================================

#[test]
fn comparison_operations() {
    let a: usize = 23;
    let b: usize = 7;

    // Equal.
    let result: bool = a == b;
    println!("{:>25} {} == {} ({})", "Equal:", a, b, result);

    // Not Equal.
    let result: bool = a != b;
    println!("{:>25} {} != {} ({})", "Not Equal:", a, b, result);

    // Greater Than.
    let result: bool = a > b;
    println!("{:>25} {} >  {} ({})", "Greater Than:", a, b, result);

    // Greater Than Or Equal To.
    let result: bool = a >= b;
    println!(
        "{:>25} {} >= {} ({})",
        "Greater Than Or Equal To:", a, b, result
    );

    // Less Than.
    let result: bool = a < b;
    println!("{:>25} {} <  {} ({})", "Less Than:", a, b, result);

    // Less Than Or Equal To.
    let result: bool = a <= b;
    println!(
        "{:>25} {} <= {} ({})",
        "Less Than Or Equal To:", a, b, result
    );
}
