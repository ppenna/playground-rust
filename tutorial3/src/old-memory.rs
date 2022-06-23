/*
    ***** Old code from Fall 2019 *****
*/

#![allow(dead_code, unused_variables, unused_assignments, unused_mut)]

// Motivating Question:
// How does Java/Python handle assingment from some non-trivial data e.g. an object?
// Talk about reference counted and garbage collection.
// Talk about deep copies.

// What about C or C++?
// Talk about implicit copies.

// What are the tradeoffs between garbage collection and the C/C++ way.
// What goes wrong when memory is freed incorrectly by the programmer?


// A value is dropped when it goes
// out of scope.
fn memory() {
    {
        let vec = vec![1, 2, 3];  // Memory allocated here.
    } // Memory is dropped here.


    fn f(){
        let my_box = Box::new(5); // Explicit heap memory
    } // dropped here

    struct Person { name: String, birth: i32}

    let p = Person {name: "omar".to_string(), birth: 1993};

    let mut vec = Vec::new();
    vec.push(p);

    // let p2 = p; // p has been consumed by push.

    // This is how rust manages memory. Lexically.
    // This means, at compile time, by analysing the code...
    // Rust always knows when a value should be dropped.
}

fn simple_move() {
    let vec = vec![1, 2, 3];
    // let vec2 = vec;
    dbg!(vec); // use of moved value: `vec`


    // If you want a actual copy of the vector use .clone() instead:
    // let vec2 = vec.clone();
}

// What happens when we do the same with simpler types like integers?
fn copy_trait() {
    let x = 3;
    let y = x;
    dbg!(x);
}

// It would be really tedious or verbose to explicitly clone numbers...
// So Rust allows us to auto copy them.

// What makes something like i32 special? How does Rust decide whether to
// allow a Copy or instead make use clone explicitly?

// If the type is a simple bit pattern in memory, it can implement Copy.
// It cannot hold data or need a constructor or destructor.

// Copies are a simple bit-wise copy of data.
// Clones may copy resources e.g. a file descriptor.

// Values are moved when they are passed to a function.
fn f(v: Vec<i32>) {
    unimplemented!();
}

fn function_move(){
    let mut vec = vec![1, 2, 3];
    f(vec);
    // vec.push(3);
}

// Use reference if you don't want to consume the value!
// More on this later...


// Values are moved when returned from a function:
fn move_around(v: Vec<i32>) -> Vec<i32> {
    v
}

// Value is moved to move_around, but moved again from returning.
// So we are allowed to use it.
fn use_move_around() {
    let mut vec = vec![1, 2, 3];
    vec = move_around(vec);
    dbg!(vec);
}

// Iterating through array moves the values.
fn for_move(vec: Vec<i32>) {
    for v in vec {
        unimplemented!();
    }

    // println!("{:?}", vec);
}

// Use reference &vec to not move the values out.
// Notice this changes the type of v from i32 to &i32.
// Which may, or may not be what you want.

// We can implelment Copy/Clone for our own type. As long as all
// our fields also implement Copy/Clone!
#[derive(Copy, Clone)]
struct Label {number: i32}

// String does not implement Copy!
// If we try, Rust will get angry.

// #[derive(Copy, Clone)]
// struct LabelString {number: String}


// RC
fn rc_example() {
    use std::rc::Rc;
    // Rust can infer all these types; written out for clarity
    let s: Rc<String> = Rc::new("rust".to_string());

    // These may look like true clones "deep copies", but instead they
    // merely create an extra alias to data.
    let t = s.clone();
    let u = s.clone();
}
