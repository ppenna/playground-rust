/*
    Lecture 3: Introduction to Ownership
*/

// Allow warnings -- to avoid clutter during tutorial
// I do NOT recommend doing this for your code
#![allow(dead_code, unused_variables, unused_assignments, unused_mut)]

fn main() {
    println!("CIS 198 Lecture 3");

    println!("Printing a vec:");
    // print_vec_1(vec![1, 3, 5, 7]);
    print_vec_2(vec![1, 3, 5, 7]);

    // let name = String::from("Caleb");
    // let place1 = String::from("Philadelphia");
    // let place2 = String::from("Mars");
    // "Philadelphia" and "Mars" are &str values
    // bad_print(name, places_lived);
    // // bad_print(name, places_lived); // doesn't work
    let name = "Caleb";
    let mut places_lived = vec!["Philadelphia", "Mars"];
    good_print(&name, &places_lived);
    good_print(&name, &places_lived);
    // Modify places_lived here
    places_lived.push("The Moon");
    good_print(&name, &places_lived);
}

/* ========== For Loops ========== */

/*
    Problem: write a for loop to print a vector
*/

fn print_vec_1(vec: Vec<usize>) {
    let n = vec.len();
    // Bad way, requires unecessary explicit indexing.
    // Try running clippy (`cargo clippy`); you will see it
    // complains about this.
    for i in 0..n {
        println!("Element v[{}] = {}", i, vec[i]);
    }
}

fn print_vec_2(vec: Vec<usize>) {
    // Nice way, iterate directly
    for x in &vec {
        println!("Element: {}", x);
    }
}

/* ========== Ownership ========== */

/*
    There is a problem with our above functions. :(
    What happens when we try to use them?
*/

#[test]
fn use_print_vec_2() {
    let mut v = vec![1, 2, 3];
    print_vec_2(v);
    // Doesn't work :(
    // Can't use v after passing ownership to function
    // v.push(4);
    // print_vec_2(v);
}

/*
    ***Ownership Rules:***

    1. Each value in Rust has a variable that’s called its owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.

    How can we fix our print function?
*/
fn print_vec_3(vec: Vec<usize>) -> Vec<usize> {
    // Nice way, iterate directly
    for x in &vec {
        println!("Element: {}", x);
    }
    vec
}

#[test]
fn use_print_vec_3() {
    let mut v = vec![1, 2, 3];
    let mut v = print_vec_3(v);
    v.push(4);
    let mut v = print_vec_3(v);
    // whatever else I want to do with v
}

/*
    Is there a better way?

    References!

    References represent *borrowing* a value.

    How is borrowing related to ownership?

    - Normal references:
        &usize, &i32, &Vec<T>, &[T], &str, ...
        I am borrowing read-only access to data
        I can't modify that data
        I don't own that data (i.e. when I stop borrowing it, it
            goes back to the owner)

    - Mutable references
        &mut usize, &mut i32, &mut Vec<T>, &mut [T], &mut str, ...
        I am borrowing read/write access to data
        I can modify the data
        I give back ownership at the end.

    Rule of borrowing in Rust:
    - You can either have any number of references to data,
      OR you can have a single mutable reference.

    Let's go back and fix our print vector function (final version!)
*/

// fn print_vec_final(vec: &Vec<usize>) {
// ^^ could do that, but reference to slice is preferable
fn print_vec_final(vec: &[usize]) {
    for x in vec {
        println!("Element: {}", x);
    }
}

#[test]
fn use_print_vec_4() {
    let mut v = vec![1, 2, 3];
    print_vec_final(&v);
    v.push(4);
    print_vec_final(&v);
}

/*
    GENERAL RULE OF THUMB:
    - Function arguments: references
    - Function return value: owned data
*/

/*
    Example with strings

    String: owned, valid UTF8, growable (modifiable) string
    &str: reference, immutable, view of valid UTF8 string data somewhere in
    memory

    Q: why is &str a reference and not just a 'str'?

    A: recall ownership rule: everything must be owned by someone

    Think of &str like &String, but it could be more general -- it's a window or
    view into memory somewhere.
*/

// In particular, &str can be completely static data compiled with your code
const IN_FILE: &str = "my_input_file.in";
const OUT_FILE: &str = "my_out_file.out";
// This data does not need to be growable/modifiable, so we don't need a String
// This is why we have &str.

/*
    str: actual raw string in memory somewhere (???!) -- Don't use this!
    - Owned by: String? the compiler / your binary?
    - Technical limitations make str hard to use directly

    When you do &s for a string s, you automatically get an &str.
*/

fn bad_print(name: String, places_lived: Vec<String>) {
    println!("Person's name is {}", name);
    println!("And they have lived in the following places:");
    for place in &places_lived {
        println!("  - {}", place);
    }
    // Has same ownership issues as print_vec_1, _2, _3
    // Not what we want.
}

fn good_print(name: &str, places_lived: &[&str]) {
    // Recall: &[T] is a slice type -- a reference to Vec<T>-like memory
    // similar to how &str is a reference to String-like data.
    println!("Person's name is {}", name);
    println!("And they have lived in the following places:");
    for place in places_lived {
        println!("  - {}", place);
    }
}

/*
    Q: How are for loops related to ownership and references?

    Answer: for loops usually iterate over a reference to data.

    - for x in &v {
    if s is a String:
    - for ch in s.chars() { // implicitly: reference to char

    When you iterate over data, you are borrowing it! So, normally
    a & reference.

    Internally: for loops are just iterators; we will get to iterators in a
    later tutorial.
*/

/*
    Exercise: Write a function which
    (1) appends two vectors
    (2) interleaves two vectors

    Design choice when writing a function:
    - what should the arguments and return values be?
*/
// Usually better to accept references
fn append_vecs(v1: &[usize], v2: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();
    for &x1 in v1 {
        result.push(x1);
    }
    for &x2 in v2 {
        result.push(x2);
    }
    result
}

// Option 2: could be more efficient if v1 is large and v2 is small?
fn append_vecs_2(v1: &mut Vec<usize>, v2: &[usize]) {
    // No longer needed!
    // let mut result = Vec::new();
    // for &x1 in v1 {
    //     result.push(x1);
    // }
    for &x2 in v2 {
        v1.push(x2);
    }
}

#[test]
fn test_append_vecs() {
    assert_eq!(append_vecs(&[1, 2, 3], &[4, 5]), vec![1, 2, 3, 4, 5]);
    let mut v1 = vec![1, 2, 3];
    append_vecs_2(&mut v1, &[4, 5]);
    assert_eq!(v1, vec![1, 2, 3, 4, 5]);
}

// Suppose we want a function which interleaves two vectors: for example, if
// the input is [1, 2, 3] and [4, 5, 6] the output should be [1, 4, 2, 5, 3, 6].
// In this case, unlike in the append case, it doesn't really make sense
// to have a mutable version -- we can't more efficiently interleave the
// two vectors if we have mutable access to one.
// So here the function signature: (v1: &[usize], v2: &[usize]) -> Vec<usize>
// makes more sense.
fn interleave_vecs(v1: &[usize], v2: &[usize]) -> Vec<usize> {
    let mut result = Vec::new();
    let n = v1.len();
    assert_eq!(v2.len(), n);
    for i in 0..n {
        result.push(v1[i]);
        result.push(v2[i]);
    }
    result
}

/* ========== Copying and cloning ========== */

/*
    General principle in Rust: you can always "opt out" if:
    - you just want something more convenient
    - you don't care about efficiency
    - you prefer data to be immutable to avoid ownership issues

    How to opt out: Cloning

    - Just call .clone() on most datatypes to get a copy
      that is separately owned
    - Some other useful functions:
        .to_owned() or .to_string() -- turn an &str into a String
        .clone()
*/

fn consume_string(mut s: String) {
    // Takes ownership of s and does something with it
    println!("{}", s);
    s += " world";
    println!("{}", s);
}

fn cloning_examples() {
    let s1: String = "hello".to_owned();
    let s2: String = "hello".to_string();
    let s3: String = s2.clone();
    let s4: String = s3.clone();
    // Each variable owns its own data
    // So we can call consume_string multiple times
    consume_string(s1);
    consume_string(s2);
    consume_string(s3);
    consume_string(s4);

    // Similar for vector -- recall the problem with
    // print_vec_2? We can get around it by cloning

    let v = vec![1, 2, 3];
    print_vec_2(v.clone());
    print_vec_2(v); // would be an error if we didn't use v.clone() above
}

/*
    Cloning is a good idea if your data is small, and a bad idea if
    your data is large since it will copy all of it.

    For simple types like integers, cloning is extremely efficient, so Rust
    offers types which clone automatically, called **Copy types**

    - i32, u32, u64, i64, usize, isize, etc.
    - normal (immutable) references and slices (e.g. &str)
    - NOT mutable references

    For these you can call .copy() instead of .clone()

    **Important:** when you pass ownership into a function, copying happens
    automatically

    For example, we don't run into any ownership problems with a print_int
    function like we do with print_vec:
*/

fn print_ints(x: usize, y: usize) {
    println!("ints: ({}, {})", x, y);
}

#[test]
fn test_print_ints() {
    let x = 1;
    let y = 2;
    print_ints(x, y);
    // x and y were copied, so the function did not take ownership.
    // Therefore, we can keep doing stuff with x and y.
    print_ints(x, x);
    print_ints(y, y);
}

// If we wanted to modify x and y, we would still need
// to write a function which takes a mutable reference, like:
// fn do_some_modification(x: &mut usize) {
// }

/*
    In general, this means that for functions taking integers, you can
    just accept an integer as argument instead of a reference.
    This is in general true for "cheaply copiable" types -- besides
    integers, other Copy types include pairs of ints like (u64, u64).

    Under the hood, references like &str and &usize are also Copy types!
    That is why they pass to functions so easily without transferring
    ownership.
*/

/* ========== Interlude ========== */
/*
    Rewriting the C++ examples from the quiz

    The nicest thing about Rust's ownership rules is that they prevent us
    from shooting ourself in the foot with memory errors, like in the C++
    examples we saw in the slides at the start of tutorial.

    If we re-implement those three examples, for all three we find that
    Rust's borrow checker (i.e. Onwnership!) prevents us from compiling
    the code.

    Try commenting out the code below and see the compiler errors!

    So while worrying about ownership can be a little bit of work, we get
    the benefit that shooting yourself in the foot is not allowed by the
    compiler.

    If you *do* want to shoot yourself in the foot, there is again a way to
    opt out: it's called "unsafe Rust".
    Unsafe rust will allow you do manage memory however you want, but
    it is an advanced feature, which is unnecessary for most everday programming
    tasks.
*/

fn ex1() {
    // Example 1: modifying a data structure while iterating over it
    // (can lead to corrupt memory)

    let mut v = vec![1, 2, 3];

    let mut j = 0;
    for x in &v {
        // Comment this out to see error message: "cannot borrow `v` as mutable
        // because it is also borrowed as immutable"
        // v.push(j);
        j += 1;
        println!("{}: {}", j, x);
    }
    v.push(3);
}

fn ex2() {
    // Example 2: freeing memory while still in use

    let mut v = vec![1, 2, 3];

    let item = v.iter().find(|&r| *r == 2).unwrap();

    v.clear();
    println!("v: {:?}", v);

    // Comment this out to see error message:
    // "cannot borrow `v` as mutable because it is also borrowed as immutable"
    // println!("Found: {:?}", *item);
}

fn ex3() {
    // Example 3: Trying to create memory without owning it (memory leak!)

    let mut grid: Vec<&Vec<isize>> = Vec::new();

    // Comment this out to see the error message:
    // "temporary value dropped while borrowed"
    // grid.push(&vec![1, 2, 3]);
    // grid.push(&vec![4, 5, 6]);
    // grid.push(&vec![7, 8, 9]);

    println!("{:?}", grid);

    grid.clear();
}

/* ========== Lifetimes ========== */

/*
    This is an advanced topic.

    If references do not own their data, how does Rust ensure that references
    are valid (memory safety)? (Recall: C++ examples with references to garbage
    somwhere in memory)

    Answer: Lifetimes.
    - 99% of the time you do not need to know about lifetimes
    - A lifetime is what it sounds like: it says how long a variable or
      reference to memory is valid (lives)
    - Rust will infer the lifetimes of variables so you don't have to!
    - You used to need to tell Rust about lifetimes more often, but the Rust
      compiler has gotten even better at inferring the lifetimes in the
      last few years.
    - Still, sometimes you will need to know how Rust is working internally and
      write what are called "lifetime annotations"
*/

// Typical example: returning a reference
// This Function doesn't compile, because it tries to create
// a reference to a variable that is then dropped at the end of the function
// You get an error that says something about lifetimes.
// fn return_ref_1(s: String) -> &str {
//     &s
// }

// This function does compile. Rust figures out that the lifetime of the
// return value is the same as the lifetime of the argument:
fn return_ref_2(s: &str) -> &str {
    s
}

// With *explicit lifetime annotations*, we would write this like this:
fn return_ref_2_explicit<'a>(s: &'a str) -> &'a str {
    s
}
// The 'a denotes a lifetime.
// Every reference &T implicitly has a lifetime, and &'a T means a reference to
// T with lifetime 'a if you need it to be explicit.
// Here is another example:
fn return_second<'a, 'b>(s1: &'a str, s2: &'b str) -> &'b str {
    s2
}
// This time we have two lifetimes, and the return value has the lifetime of the
// second thing.

// The special 'static lifetime can be used for data that is permanent --
// essentially, data which lives forever. Generally you want to avoid this,
// since the data may clutter up memory.
fn string_from_static_str(s1: &'static str) -> String {
    String::from(s1)
}
// Q: Why can't you convert the other way around, from a String to a 'static
// str?
// A: Because no one would own the memory -- remember that all memory must be
// owned.
// The usual use case for static strings is constants:
const STATIC_STR: &'static str = "THIS IS A STATIC STRING";

// In the above case, we also don't need 'static in the argument, this is just
// illustrative.
// In particular we could write
fn string_from_str_1<'a>(s1: &'a str) -> String {
    String::from(s1)
}
// or just
fn string_from_str_2(s1: &str) -> String {
    String::from(s1)
}
// string_from_str_1 and _2 are equivalent. Rust infers the <'a> and 'a
// annotations in the second version.

// In fact, Clippy (run `cargo clippy`) will complain about the code we've
// written above, since we wrote explicit annotations 'a when they are not
// necessary.
