/*
    Closures in Rust
*/

use std::fmt::Debug;

/*
    QUIZ

    Q1. What is the difference between get_last_1 and get_last_2 below?
    Q2. What is the difference between get_last_1 and get_last_3 below?

    A1. .expect() panics, .unwrap_or provides a default.
    A2. get_last_1 is a bad implementation :( It prints the println!s even
        if the vector is empty, because it evaluates the expression arguments
        before passing them to .unwrap_or.
        get_last_3 will ONLY evaluate the || {} object if the vector is actually
        empty.
        The || {} thing is called a closure.
        It's passed to unwrap_or_else as a function, not as a value.

    In other words, the argument in get_last_3 is evaluated lazily.

    Note: Clippy is partly smart on this, actually warns us about the evaluated
    function call in get_last_2, but not in get_last_1.
*/

pub fn get_last_1(v: &[usize]) -> usize {
    // .copied() converts from Option<&T> to Option<T>
    v.last().copied().unwrap_or({
        println!("Warning: couldn't get last element of vec: {:?}", v);
        println!("Returning 0 instead");
        0
    })
}
#[allow(clippy::expect_fun_call)]
pub fn get_last_2(v: &[usize]) -> usize {
    v.last()
        .copied()
        .expect(&format!("Warning: couldn't get last element of vec: {:?}", v))
}
pub fn get_last_3(v: &[usize]) -> usize {
    v.last().copied().unwrap_or_else(|| {
        println!("Warning: couldn't get last element of vec: {:?}", v);
        println!("Returning 0 instead");
        0
    })
}

/*
    What are closures?

    Two syntaxes:
    1.
        |arg1, arg2, ...| {
            /* function body */
        }
    2.
        move |arg1, arg2, ...| {
            /* function body */
        }
*/

pub fn closure_examples(input: Option<String>) {
    // We want to create some function that will be
    // evaluated on error (i.e. if the input is empty)
    // but NOT evaluted right when we define it.
    let c1 = || "Error: value was None".to_string();
    let c2 = || format!("Error: value was None: {:?}", input.clone());

    println!("{}", input.clone().unwrap_or_else(c1));
    println!("{}", input.clone().unwrap_or_else(c2));
}

pub fn closure_examples_move(input: Option<String>) {
    // The previous examples did not capture any part of the environment.
    // Part of what a closure can do is capture the environment.

    // Captures diagnostic_data
    let diagnostic_data = vec!["Stack size: 3", "Function name: 'examples'"];
    let c1 = move || {
        format!("Error: value was None; Diagnostic data: {:?}", diagnostic_data)
    };
    // Rust figures out that ownership of diagnostic_data should be transferred to
    // the closure, but we can make that explicit with move.

    let mut diagnostic_data_2 =
        vec!["Stack size: 3", "Function name: 'examples'"];
    let c2 = move || {
        // Takes ownership over diagnostic_data_2 (needs mutable access)
        format!(
            "Error: value was None; Diagnostic data: {:?}",
            diagnostic_data_2.drain(..)
        )
    };
    // This one causes an error because we try to move ownership to two
    // different closures.
    // let c3 = move || {
    //     format!("Error: value was None; Diagnostic data: {:?}", diagnostic_data_2.drain(..))
    // };

    println!("{}", input.clone().unwrap_or_else(c1));
    println!("{}", input.unwrap_or_else(c2));
}

pub fn closure_with_arguments_example() {
    // Closure with two arguments
    let add_2_nums = |num1, num2| num1 + num2;
    println!("1 + 2 = {}", add_2_nums(1, 2));
    println!("1 + 3 = {}", add_2_nums(1, 3));

    let option = None;
    let on_error = || {
        // If the option is Some, this stuff isn't evaluated at all!
        format!("1 + 4 = {}", add_2_nums(1, 4))
    };

    // Does ownership of add_2_nums get passed to the above closure?
    // A: No, it only gets a reference to add_2_nums.
    // Recall the rule: multiple immutable references, only one mutable reference
    // and only one owner at a time.
    println!("1 + 5 = {}", add_2_nums(1, 6));

    println!("{}", option.unwrap_or_else(on_error));
}

/*
    To summarize:

    - Bottom line: move means transfer ownership into the closure explicitly;
      sometimes Rust will figure that out for you, but if you are having borrowing errors,
      try adding move to your closures.

    - Just like any other object in Rust, closures can be passed around, have owners,
      can have multiple immutable references (or one mutable reference), have
      lifetimes, and have inside them references to other data.
*/

/*
    Closures often come up when writing error messages
    (as in .unwrap_or_else above), or when using third-party
    libraries with complex IO.

    How do we write our own functions like .unwrap_or_else ?

    ** Functions taking functions as arguments **

    Sometimes we noctice that we have duplicated code
    with essentially the same form or pattern. Examples:
*/

pub fn add_1_to_each(v: &mut Vec<usize>) {
    for x in v.iter_mut() {
        *x += 1;
    }
}

pub fn zero_rest_each(v: &mut Vec<usize>) {
    for x in v.iter_mut() {
        println!("Resetting: {}", x);
        *x = 0;
    }
}

/*
    Both of these functions are kind of doing the same thing...
    How can we unify these examples?
*/

pub fn do_for_each<F>(v: &mut Vec<usize>, f: F)
where
    F: Fn(&mut usize),
{
    for x in v.iter_mut() {
        f(x)
    }
}

pub fn add_1_to_each_using_do_for_each(v: &mut Vec<usize>) {
    // To use a function which takes a function as argument, as we saw
    // earlier with .unwrap_or_else(),
    // you pass a closure.
    do_for_each(v, |x| *x += 1);
}
pub fn zero_rest_each_using_do_for_each(v: &mut Vec<usize>) {
    do_for_each(v, |x| {
        println!("Resetting: {}", x);
        *x = 0
    });
}

/*
    What's going on? Fn is called the Function trait.
    Meaning of the Fn trait?

    "Something that behaves like a function"
    - Closures behave like functions
    - Actual functions (in your code) (aka function pointers, just as in C)
      behave like functions
    - Maybe some other types also behave like functions, if so, they can implement
      the Fn trait to tell Rust "I can be passed as a function with input type Input and
      output type Output."
*/
trait MyFn {
    // Associated types for the trait
    // (Implementor of the trait needs to specify these)
    type Input;
    type Output;

    fn apply(&self, input_val: Self::Input) -> Self::Output;
}

// Any type could implement this in theory...
// This doesn't look like a function!
// But it behaves like one.
pub struct MyAddxStruct {
    name: String,
    value_to_add: usize,
}
impl MyFn for MyAddxStruct {
    type Input = usize;
    type Output = usize;
    fn apply(&self, input_val: Self::Input) -> Self::Output {
        println!("Hello from my function-like struct: {}", self.name);
        input_val + self.value_to_add
    }
}

// Recall: zero-cost abstraction
// Internally: there will be a **copy** of do_for_each
// for each type that implements the function trait.
// This is the advantage of using generic code do_for_each<F>
pub fn do_addx_for_each(v: &mut Vec<usize>, add_x: MyAddxStruct) {
    do_for_each(v, |x| *x = add_x.apply(*x));
}

/*
    EXERCISES
    To put our knowledge to use:

    1. Function which does something twice

    2. Function which does something and logs the input/output

    3. Function which does something for each element of a list
    (Skipped this one)
*/

pub fn do_twice<F, X, Y>(action: F, input: &X) -> (Y, Y)
where
    F: Fn(X) -> Y,
    X: Clone,
{
    println!("Applying the action twice...");
    let y1 = (action)(input.clone());
    let y2 = (action)(input.clone());
    (y1, y2)
}

pub fn log_input_output<F, X, Y>(action: F, input: &X) -> Y
where
    F: Fn(&X) -> Y,
    X: Debug,
    Y: Debug,
{
    println!("Calling function with input: {:?}", input);
    let result = action(input);
    println!("Function output: {:?}", result);
    result
}

/*
    Alternatives to the Fn Trait

    - FnOnce: a function that can only be called once (consumed)
    - FnMut: a function which mutates itself in some way when called

    Closures can be interpreted as any/all of these, and when
    writing a function which takes a function as argument,
    you want to think about which type of function your argument
    should be.

    For example, do_twice can't accept an FnOnce,
    but it could accept an FnMut if you so desired.
    And log_input_output can accept FnOnce (and probably should to be more general).
*/

/*
    ========== Start of Lecture 7 Part 2 ==========
*/

/*
    QUIZ 1: What does the following code print?
*/

pub fn use_fn<F: Fn()>(f: F) {
    println!("Hello 1!");
    f();
}

#[test]
pub fn test_use_fn() {
    let closure = {
        let two = "2";
        let three = "3";
        println!("Hello {}!", two);
        move || println!("Hello {}!", three)
    };
    use_fn(closure);

    // To show output from the test
    // assert!(false);
}

// Note on 'move'
// If 'move' is removed in the above code, it fails to compile
// Move is used to capture local variables and data and "move" ownership
// of them into the closure. So, a closure, unlike a normal function, owns and
// contains some information about its environment, e.g. data that is needed
// inside the closure.

/*
    QUIZ 2: What will happen if we try to compile this code?
*/

pub fn example_do_all_broken<F>(actions: Vec<F>)
where
    F: Fn() -> String,
{
    for (i, action) in actions.iter().enumerate() {
        println!("Result of action {}: {}", i, action());
    }
}

// #[test]
// fn test_example_do_all() {
//     let action1 = || {
//         format!("2 + 2 = {}", 2 + 2)
//     };
//     let action2 = || {
//         let name = "Caleb";
//         format!("Hello, {}", name)
//     };
//     example_do_all_broken(vec![action1, action2]);
//     // Type error!
//     // action1 and action2 are not the same type! They are different functions,
//     // different data, different size, etc.
// }

/*
    Problem: every function (or closure) is its own type!

    This is feature, not a bug -- because each function is its own type,
    Rust doesn't have to dynamically figure out what a function is at runtime;
    it knows at compile time exactly what code and data (for closures)
    a function contains, how to evaluate the function, the function's lifetime,
    etc.

    (Limitation of the Fn(args) -> return value trait.)

    Though the Fn trait is most ideal, Rust has other ways to specify
    function arguments.

    - Dynamic function objects

      Box<dyn Trait>
      Dynamic object which implements Trait.
      -> "Dynamic" meaning that the exact type of the object is stored and looked up
         at runtime. (E.g. like a Python object.)

    - Function pointers

    EXERCISE:
    4. Rewrite example_do_all in a way that works.
*/

pub fn example_do_all_fixed(actions: Vec<Box<dyn Fn() -> String>>) {
    for (i, action) in actions.iter().enumerate() {
        println!("Result of action {}: {}", i, action());
    }
}

#[test]
fn test_example_do_all_fixed() {
    let action1 = || format!("2 + 2 = {}", 2 + 2);
    let action2 = || {
        let name = "Caleb";
        format!("Hello, {}", name)
    };
    example_do_all_fixed(vec![Box::new(action1), Box::new(action2)]);
    // Type error!
    // action1 and action2 are not the same type! They are different functions,
    // different data, different size, etc.
    // assert!(false);
}

/*
    Useful syntax for trait bounds

    Trait bounds can get a bit hard to read,
    and there are some ways to improve the syntax:

    - The Where syntax
      See do_twice and log_input_output for how this is used.

      fn example<A: Trait1, B: Trait2, F: Fn(...) -> ...>(a: A, b: B, f: F) -> { ... }

      fn example<A, B, F>(a: A, b: B, f: F)
      where
          A: Trait1,
          B: Trait2,
          F: Fn(...) -> ...,
      {
          ...
      }

      I usually prefer 'where' clauses, but for simple functions like
      pub fn clone_x<X: Clone>(x: X) {
          x.clone();
          x.clone();
      }
      I don't bother with where.

      With multiple traits, function traits like say Fn(usize, Vec<usize>) -> String
      the first syntax gets quite clunky.

      Multiple trait bounds on the same type variable with '+':

      pub fn example<X>(x: X)
      where
          X: Clone + Debug + Eq,
      {
          assert_eq!(x.clone(), x.clone());
      }

    - The impl Trait syntax

      In place of a type argument like F, or a return value, you can put
      'impl Trait'
      'impl Clone'
      'impl Fn() -> String'

      Compiler figures out what you mean and generates the trait bound code for you

    EXERCISE:
    5. Write a function for a vector which applies a closure to every element.
    6. Write a function for a vector which returns a "print-and-clear" closure.
*/

// Syntax we are used to
// pub fn apply_to_all<X, F, Y>(v: &[X], f: F) -> Vec<Y>
// where
//     F: Fn(&X) -> Y,

pub fn apply_to_all<X, Y>(v: &[X], f: impl Fn(&X) -> Y) -> Vec<Y> {
    let mut result = Vec::new();
    for x in v {
        result.push(f(x));
    }
    result
}

// For an input argument, this is just convenience
// But for a return argument, it's actually a powerful feature that you can't
// really do without
pub fn return_print_and_clear(mut v: Vec<usize>) -> impl FnOnce() {
    // Return a closure
    // Move the vector v into the closure
    move || {
        for &x in &v {
            println!("{}", x);
        }
        v.clear();
        drop(v); // not necessary to write explicitly
    }
}

// Recap: impl Trait means:
// - I know this is a type that implements Trait, but I don't know
//   (or don't want to specify) exactly what type it is.
//   So, please figure it out for me.
