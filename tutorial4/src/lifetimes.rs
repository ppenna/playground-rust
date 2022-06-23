/*
    Topic 1 for today: Lifetimes
*/

/*
    Recall the following from last class period:

    - Ownership: Each object should have exactly one
      one owner; memory is dropped when the owner leaves scope

    - Borrowing: When you "borrow" ownership, you get either
      read-only access (any number of borrows) XOR read-write
      access (exactly one borrow) (note the XOR).

    This solves various problems related to memory safety, for example:
    - Memory leaks:
      solved by --> "memory is dropped when the owner leaves scope"
    - Null pointer dereference and pointers to bad memory
      solved by --> "Each object should have exactly one one owner"
          "memory is dropped when the owner leaves scope"
          ... but is it?

    LIFETIMES

    The problem with the above is that it doesn't allow me to ensure
    that references are always valid.

*/

pub fn copy_string(s: &str, n: usize) -> String {
    let mut result = String::new();
    for _i in 0..n {
        result += s;
    }
    result
}

#[test]
pub fn test_copy_string() {
    assert_eq!(copy_string("hello", 3), "hellohellohello");
}

// But the memory could have gone out of scope... how would we detect that?

#[test]
#[allow(unused_variables, unused_assignments)]
pub fn test_copy_string_bad() {
    let mut r: &str = "goodbye";
    {
        let s = String::from("hello");
        r = &s;
    } // lexical scope: manages the memory implicitly
      // implicitly we are dropping s here.

    // If you understand this example, then you understand lifetimes!
    // assert_eq!(copy_string(&r, 3), "hellohellohello") // doesn't work
    // How does Rust know that r is not a valid reference here?

    // Answer: it will assign an implicit lifetime to each variable.
    // A "lifetime" is just an internal thing, in Rust denoted with
    //                    'a
    // or:                'b
    // or the special:    'static
    // What it means is "how long" the reference points to valid memory;
    // it's derived from the scope of the original owner.
}

#[allow(clippy::needless_lifetimes)]
pub fn copy_string_withlifetimes<'a>(s: &'a str, n: usize) -> String {
    let mut result = String::new();
    for _i in 0..n {
        result += s;
    }
    result
}

pub fn constant_str() -> &'static str {
    "constant"
}

pub fn constant_str_dummyargs<'a>(_s1: &'a str, _s2: &'a str) -> &'a str {
    // Normally return value would be String, but let's try it with
    // an &str.
    // let s = format!("{} {}", s1, s2);
    constant_str()
}

pub fn nested_lifetimes() {
    let s = "hello";
    // s lifetime: 'static
    {
        let a = String::from("x");
        // a liftetime: 'lifetime1
        {
            let b = String::from("y");
            // b lifetime: 'liftime2
            // where we have an implicit ordering:
            // 'static >= 'lifetime1 >= 'lifetime2

            // Here s, a, and b are in scope
            println!("{}", constant_str_dummyargs(&a, &b));
            println!("{}", constant_str_dummyargs(&a, s));
            println!("{}", constant_str_dummyargs(s, &b));
        }

        println!("{}", constant_str_dummyargs(&a, s));
        // Here s and a are in scope
    }

    // Here only s is in scope.
    println!("{}", constant_str_dummyargs(s, s));
}

// Let's do one last example where lifetimes actually show up a bit more practically.

pub fn find_a<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // Return the first character of s1 or s2 if it is an a,
    // else return "".
    // (Better to return Option, but we will get to that shortly.)

    // Note: don't usually need to implement such stuff yourself,
    // they are all built-in, use the &str documentation.

    // To iterate through a &str, use .chars():

    if s1.starts_with('a') {
        // equivalent but less idiomatic: s1.chars().next() == Some('a')
        &s1[0..1]
    } else if s2.starts_with('a') {
        &s2[0..1]
    } else {
        ""
    }
}

#[test]
#[allow(unused_variables, unused_assignments)]
pub fn nested_lifetimes_2() {
    let s = "hello";
    // s lifetime: 'static
    {
        let a = String::from("x");
        // a lifetime: 'lifetime1

        let mut c: &str = "";
        {
            let b = String::from("y");
            // b lifetime: 'liftime2
            // where we have an implicit ordering:
            // 'static >= 'lifetime1 >= 'lifetime2

            // Here s, a, and b are in scope
            println!("{}", find_a(&a, &b));
            println!("{}", find_a(&a, s));
            println!("{}", find_a(s, &b));

            c = find_a(s, &b);
        }
        // b drops the memory
        // c wasn't responsible for the memory, so doesn't drop it
        // But, c is out of scope here and the Compiler can figure that out.
        // println!("{}", c);

        println!("{}", find_a(&a, s));
        // Here s and a are in scope
    }

    // Here only s is in scope.
    println!("{}", find_a(s, s));
}
