/*
    CIS 198: Rust Programming

    Lecture 2: basic syntax, types, and expressions

    src/main.rs
*/

// Allowing warnings, since we wrote a lot of illustrative / playground code
#![allow(dead_code, unused_variables)]
#![allow(clippy::unused_unit, clippy::unnecessary_cast, clippy::needless_return)]

/*
    Tour of Cargo

    Creating a repository:
    - cargo new or cargo init
    - Manage with git!

    Building and running:
    - cargo build
    - cargo build --release
    - cargo run
    - cargo run --release
    - cargo run --bin <binary name>
    - cargo run --bin <binary name> --release
    - cargo test
    - cargo test --release

    Clippy and rustfmt:
    - cargo clippy
    - cargo fmt

    Package dependencies:
    - Cargo.toml
    - Cargo.lock
*/

/*
    Rust Syntax

    - Often similar to C++
    - Diverges in some important ways to be more readable / clear
    - Sometimes similar to functional languages (OCaml)

    Some themes for today:

    1. Explicit is better than implicit

    2. Strong types: every variable has a type

    3. Explicit memory: every variable has a known memory location

    4. Passing by reference

    (Recall: safety and zero-cost abstractions)

*/

/*
    Main function

    The easiest way to run your program: include a file
        src/main.rs
    and a function
        fn main()
*/
fn main() {
    println!("Hello, CIS198!");

    // This is a comment
    /* This is also a comment */

    machine_types();

    println!("2 + 2 = {}", add_two_integers(2, 2));
    println!("2 + 3 = {}", add_two_integers(2, 3));
    println!("2 + -2 = {}", add_two_integers(2, -2));

    // array();

    // slices();

    strings();
}

/*
    Basic types and expressions
*/

// In Rust we specify the types of integers.
// What are the tradeoffs of using different types?
// Peformance and memory size.
#[allow(clippy::many_single_char_names)]
fn machine_types() {
    let n: i8 = 0;
    let m: i16 = 1;
    let o: i32 = 0;
    let p: i8 = 0;

    // Quick debugging tutorial: printing and assertions
    // What happens if we print these out?

    // println!("n = {}, m = {}, o = {}, p = {}", n, m, o, p);

    // assert!(n == m as i8);
    // debug_assert!(n == p + 1);
    // assert_eq!(n, p + 1);
    // debug_assert_eq!(n, p + 1);

    // Different size literals
    let p1 = 0i64;
    let p2 = 0 as i64;
    let p3 = p2 as i128;
    let p4 = p3 as i64;

    // Unsigned versions.
    let n = 0u32;
    let m: u128 = 0;
    // println!("{} {}", n, m);

    // Machine dependent types
    let n: isize = 40;
    // Indexer!
    // let i: usize = 5;
    let i = 0;

    let array: [i32; 3] = [1, 2, 3];

    println!("{}", array[i]);
    assert_eq!(i, 0 as usize);

    // Why do we have specific sizes instead of the C/C++ approach?
    // Implicit sizes are not good enough for systems programming, where the size
    // in bytes matters.
    // https://stackoverflow.com/questions/11438794/is-the-size-of-c-int-2-bytes-or-4-bytes

    // Why doesn't C specify a size for integers?

    // Whats happens if we try to use a non usize for an array index?
    // let i: isize = 1;
    // println!("{}", array[i]);
    // println!("{}", array[i as usize]);

    // Why are array indices usize i.e machine dependent?
    // Same reason C doesn't specify size. To allow flexibility with the machine
    // dependent code.

    // Chars and bools are not ints!
    let b1 = true;
    let b2: bool = true;
    let c1 = 'c';
    let c2: char = 'c';

    // Why don't we need explicit type annotations

    // Type error
    // assert_eq!(b1, c1);

    assert_eq!(b1, b2);

    // Pretty printing
    println!("{} {} {} {}", b1, b2, c1, c2);
    // Debug printing
    println!("{:?} {:?} {:?} {:?}", b1, b2, c1, c2);

    // Recall principles:
    // Explicit is better than implicit!
    //    --> conversion between types
    // Strong types
}

// Functions
pub fn add_two_integers(a: isize, b: isize) -> isize {
    let result = a + b;
    return result;
}

// Mutability
pub fn add_two_integers_v2(a: isize, b: isize) -> isize {
    let mut result = 0;
    result += a;
    result += b;
    return result;
}

pub fn add_two_integers_result_by_reference(a: isize, b: isize, result: &mut isize) {
    *result = 0;
    *result += a;
    *result += b;
}

pub fn add_two_integers_v3(a: isize, b: isize) -> isize {
    let result = 0;
    let mut result = result;
    add_two_integers_result_by_reference(a, b, &mut result);
    return result;
}

// If expressions
pub fn absolute_value(a: isize) -> usize {
    if a > 0 {
        // return a as usize;
        a as usize
    } else {
        // return -a as usize;
        -a as usize
    }
}

// This time, instead of using main, let's use unit tests.
#[test]
pub fn test_absolute_value() {
    assert_eq!(absolute_value(0), 0);
    assert_eq!(absolute_value(3), 3);
    assert_eq!(absolute_value(-5), 5);
    assert_eq!(absolute_value(-3 as isize), 3 as usize);
}

pub fn absolute_diff(x: isize, y: isize) -> usize {
    absolute_value(x - y)
}

// Expressions vs return
// Ifs are expressions, not statements.
// What is the difference between an expression and statement?
pub fn if_expr(b: bool) {
    let x: i32 = if b { 5 } else { 10 };
    let y = 3;
    if b {
        6
    } else {
        11
    };
    println!("{}", x);
}

/*
    Arrays, vectors, and slices
*/

pub fn array() {
    // Size is hardcoded for arrays.
    // There space is allocated directly in the binary.
    // Probably in the .data or .rodata sections of an ELF file.
    let a = [1, 2, 3];
    let zeroes = [0; 1000];
    let b: [i32; 3] = [1, 2, 3];

    // Typical example
    let input_files = ["input/input_1.txt", "input/input_2.txt"];

    // Instead, Vectors are dynamically allocated.
    // Talk about heap and stack, and "pointers".
    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", v);
    v.push(3);
    println!("{:?}", v);
    v.pop();
    println!("{:?}", v);
    v.push(5);
    println!("{:?}", v);
    v.push(6);
    println!("{:?}", v);

    println!("{:?}", a);
    println!("{:?}", zeroes);
    println!("{:?}", b);
    // println!("{:?}", a + a);
    // a.push(3);

    // Rust vectors have their capacity and length fields
    // as part of the vector size, accessing elements requires
    // following the pointer down to the heap.

    // How is this different from Objects in other languages?
}

pub fn slices() {
    let a = [1, 2, 3];
    // Fat pointer: pointer to heap, length, capacity,
    let v = vec![1, 2, 3];

    // Take a reference to the address of the vector.
    // We call this a _slice_, notice the type has changed to
    // resemble an array.

    // Arrays and Vector references kinda have the same memory layout...
    let v_slice: &[i32] = &v;
    let a_slice: &[i32] = &a;
    // So we can take references to both! (There is some automatic conversion from
    // &Vec<i32> to &[i32])...

    // How big are references?
    // Probably the size a pointer.

    // Accept a vector:
    // Write functions that can take either.
    fn print(s: &[i32]) {
        println!("{:?}", s);
    }

    // Same function prints both arrays and vector references.
    print(v_slice);
    print(a_slice);
    // Rust knows how to convert vectors to slices.
    print(&v);
    print(&v[1..2]);
}

/*
    Strings

    Handling of strings is a major security risk!
    Examples: https://owasp.org/www-project-top-ten/
    Why are strings so hard to get right?
*/

pub fn strings() {
    let s1: &str = "This is a string";
    let s2: &str = "This is a string with spaghetti 🍝";

    // Uh oh...
    let s3: &str = "This is also a s̴̝͍̥̅̇̌͛̊́͊̾̓̃͆̎̏̔͠tring";

    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);
    println!("{} {} {}", s1.len(), s2.len(), s3.len());

    // The problem with C style char*:
    // Not obvious if it's modifyiable.

    // Where does this reference live
    // .data
    let s: &str = "rust";
    // ^ inmutable.

    // Rust seems to have several types of strings: str, &str, and String.
    // Why do we do this? People don't like this about rust.
    // But like everything else, Rust has good reason!

    // Heap allocated, string.
    // UTF-8, growable, modifyiable, guaranteed to be correct UTF-8
    let s: String = "rust".to_string();

    // Example from O'Reilly book
    let noodles: String = "noodles".to_string();
    let oodles: &str = &noodles[1..];
    let poodles = "ಠ_ಠ"; // Pre allocated read-only memory.

    // Notice `str` by _itself_ is a type! What does this mean?
    // It's called an "unsized type".
    // Why don't languages like Java or Python have this issue?

    // String are laid out in pretty much the same way as arrays.
    // Notice the analogies between strings and &str:
    // String ~~ Vec<T>
    // &str ~~ &[T]
    // str ~~ [T]
}

pub fn structured_types() {
    // Tuple
    // How are tuples different than arrays?
    // - Heterogenous Data
    // Number of elements is inmutable.
    let t = (5, "cat");
    // Tuple Accessor
    let num = t.0;
    let str1 = t.1;

    // int pipe(int pipefd[2]);
    // pub fn pipe() -> Result<(RawFd, RawFd)>

    // Struct (More on structs later)
    struct Coordinate {
        x: f32,
        y: f32,
    }

    // Enums (more on enums later)
    enum Weekdays {
        Monday,
        Tuesday,
        Wednesday,
    }
}

pub fn references() {
    // Think of them as C/C++ pointers, but safe!
    // Always refer to valid data.

    // A reference lives in the stack of the current function, but
    // may point to data in stacks "above" or somewhere in the address space.
    let x = 3;
    let r = &x;
    let y = &3;

    // First version.
    // fn add1(r: &i32) {
    //     r += 1;
    // }

    // Second version
    // fn add1(r: &i32) {
    //     *r += 1;
    // }

    // Working version
    // fn add1(r: &mut i32) {
    //     *r += 1;
    // }

    // add1(r);
}

pub fn unit_type() {
    // Implicitly this is the same as:
    // pub fn unit_type() -> ()

    //  What is the return type of the print function?
    let r: () = println!("hello");

    // Type of this function?
    return ();
}

#[allow(clippy::while_immutable_condition)]
pub fn while_loop(b: bool) {
    while b {
        unimplemented!()
    }

    loop {
        println!("Infinite Loop");
        // Do things.
    }
}

pub fn for_loop() {
    // Using ranges, exclusive
    for i in 0..20 {
        println!("{}", i);
    }

    // Inclusive
    for i in 0..=20 {
        println!("{}", i);
    }

    let v: Vec<&str> = vec!["cis", "198", "rust", "programming"];

    // Do not iterate over i with a index. Instead use iterators!
    for e in v {
        let e2: &str = e;
        // Do stuff with v
    }

    // What are the problems with indexing arrays with [i]?
    // Must (should) do bounds checking for array.
    // Instead the compiler is able to omit bounds checking, as it generates
    // the iteration code itself.
}

pub fn function_calls(v: Vec<i32>) {
    // Types can have methods:
    let v2 = v.clone();
    // Or associated functions.
    let mut v3: Vec<i32> = Vec::new();
    v3.push(4);
    println!("{:?}, {:?}", v, v3);

    // If compiler cannot infer the type of a generic function, turbofish
    // syntax may be required:
    // turbo.fish
    let v4 = Vec::<i32>::new();

    // Lets see what documentation looks like:
    // https://doc.rust-lang.org/std/collections/struct.HashMap.html
    // use std::collections::HashMap;
}

// Initial code. Showing all explicit types.
pub fn build_vector_rough() -> Vec<i16> {
    let mut v: Vec<i16> = Vec::<i16>::new();
    v.push(10i16);
    v.push(20i16);
    v
}

// Question: Can we omit the type of the function? Answer: No

// Omiting types:
pub fn build_vector() -> Vec<i16> {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);
    v
}

// Final version.
pub fn build_vector_final() -> Vec<i16> {
    vec![10, 20]
}
