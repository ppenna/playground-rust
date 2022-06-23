/*
    ***** Old code from Fall 2019 *****
*/

// Use mut reference to change values of pointee.
fn push_vec(vec: &mut Vec<i32>) {
    (*vec).push(3);
    // Remember that . operator automatically dereferences.
    // vec.push(3);
}

fn use_print_vec() {
    let mut vec = vec![1, 2, 3, 4, 5, 6];
    print_vec(&vec);

    vec.push(7);
    push_vec(&mut vec);
}

fn method_mut_borrow() {
    let mut v = vec![1973, 1968];
    // Some methods also borrow mutably.
    v.sort(); // implicitly borrows a mutable reference to v

    (&mut v).sort();
}

// We can have as many & to a variable as we want.
// We can only have a single mutable reference!
// And we cannot have multiple mutable references with non-mutable references.
fn borrowing1() {
    let mut x = 10;
    let y: &i32 = &x;
    let z: &i32 = &x;

    let z2 = &mut x;

    // dbg!(z);
    // dbg!(z2);
}


// We cannot move a value that is borrowed!
fn borrowing2() {
    let mut vec = vec![1, 2, 3, 4, 5];
    let r = &vec;

    // consume(vec);

    // What would happen if we were allowed to use r here? Dangling reference!
    // This happens in C/C++ all the time.
    // Newer version of C++ might not have this issue as much... idk...
}

fn consume(v: Vec<i32>){
    unimplemented!();
    // Remember the memory for v (formerly vec) is dropped here.
}

// Borrowing a reference lifetime.
fn reference_lifetime_bad() {
    let r: &i32;
    {
        // Where does x go out of scope?
        let x = 10;
        // r = &x;
    } // here

    // assert_eq!(*r, 10);
}

// Rust attempts to match Lifetimes for each variables. Lifetimes exists only at
// compile time, at runtime, everything is just a pointer.
fn reference_lifetime_ok() {
    let x = 10;
    {
        let r: &i32 = &x;
        assert_eq!(*r, 10);
    }
} // x is in scope until here. No problem using it.

fn vec_ref(){
    let vec = vec![1, 2, 3, 4];
    // Same lifetime, as vec.
    let r = &vec[1..]; // We have borrowed a part of vec here. We cannot move out vec.
}

// Writing explicit lifetimes.
// String literals are special. They live for the lifetime of the entire program.
// We use the special lifetime name 'static to refer to this lifetime.
fn static_ref(){
    let s: &'static str = "hello";
}

// Rust infers the lifetimes, let's make them explicit!
fn find_less_than<'a>(v: &'a Vec<String>, less_than: &'a String) -> Vec<&'a String> {
    let mut return_vector = vec![];
    for s  in v {
        if s < less_than {
            return_vector.push(s);
        }
    }

    return_vector
}

fn use_find_less_than(string: &String) {
    let vec: Vec<String> =
        vec!["cis198".to_string(), "wednesday".to_string(), "10:30".to_string()];

    let res: Vec<&String> = find_less_than(& vec, &string);
    println!("{}", string);
}

// Both C++ and Java suffer for iterator invalidation.
use std::collections::HashMap;
fn iterator_modification(table: &mut HashMap<i32, i32>) {
    for (k, v) in table {
        if *v < 0 {
            // table.remove(k);
        }
    }
}


// Dangerous c++ code:
// vec<string> v = {"bar", "cat", "foo"};
// auto& r = &v[2];
// v.push("foobar");
// How can this C++ code fail?

fn ref_move() {
    let mut v = vec!["bar", "cat", "foo"];
    let r = &v[1..]; // borrowing
    // v.push("foobar");

    dbg!(r);
}
