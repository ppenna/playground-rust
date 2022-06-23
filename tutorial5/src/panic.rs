/*
    Simplest form of erroring: Panic:
    unrecoverable errors: out of bounds accesses, division by zero, calling unwrap on None,
    assertion failure.
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn panics() {
    // Recall: ! means a macro
    // panic!("Error description here");

    // Works with format as well:
    let x = 5;
    let problem = "there was a problem";
    panic!("Error: {}, {}", x, problem);
}

/*
    Safety of panic:
    Panic unwind the stack, dropping variables in reverse order from declaration.
    Cleans up resources! This is a good thing!
    This is not a crash, it's well defined. Similar to Java Runtime Exception.
*/

/*
    Other forms of panic:

    - unimplemented!()
    - unreachable!()
    - assert!(false)
    - todo!()
    - .unwrap()
    - .expect()
    - etc.
*/

pub fn not_implemented_1() {
    unimplemented!()
}

pub fn not_implemented_2() {
    todo!()
}

pub fn not_reachable() {
    // useful for e.g. option types
    let x = Some(3);
    match x {
        Some(y) => println!("x = Some({})", y),
        // match statements must be exhaustive!
        // So I can't omit this case.
        None => unreachable!(),
    }
    // Can also do this with println!("x = Some({}), x.unwrap())
    // see below
}

// More interesting one: unwrap
// There are two main forms of unwrap:
// - One is on Option<T>
// - The other is on Result<T, U> which we will see more of shortly

pub fn capitalize_firstchar(s: &str) -> String {
    // To iterate over the characters in a string,
    // you use s.chars()
    // Recall:
    // - iterating using indices s[0], s[1], s[2] would not work
    //   and not be safe because of UTF-8/ unicode/ etc., so Rust
    //   forces us to use .chars() to iterate over the unicode / UTF-8 encoded
    //   characters.
    //   Sadly real-life strings are not just ASCII :(
    let mut chars = s.chars();
    // let ch = chars.next(); // might be Some or might be None
    // If we for some reason know that s is nonempty, then we can use .unwrap()
    // let ch = chars.next().unwrap();

    // But more likely we don't know s is nonempty, so we should provide
    // a more helpful error message
    // .expect()
    let ch = chars
        .next()
        .expect("capitalize function expected a nonempty string");

    format!("{}", ch.to_uppercase())
}

/*
    When to use unwrap vs expect?

    - You should use unwrap when you actually know that an Option<T> is Some(x).
      Analagous to the unreachable!()
    - If the None case is either definitely possible or might be possible if the
      caller / user / some other file uses your function wrong, then always
      better to do something more explicit, e.g. .expect() or .panic()

      We will see that even these are not ideal, the best is to return a Result<>
*/

/*
    Real-life extended example: Password struct
*/

const MIN_PASS_LEN: usize = 5;

pub struct RawPassword {
    pub user: String,
    pub pass: String,
    pub salt: String,
}
// We have defined our own type
// First we want to implement some basic initialization / getters / setters
impl RawPassword {
    pub fn new_password(user: &str, pass: &str, salt: usize) -> Self {
        // 'Self' = RawPassword
        // Suppose we want to validate that the password is good.
        let result = Self {
            user: String::from(user),
            pass: String::from(pass),
            salt: format!("{}", salt),
        };
        result.validate_is_good();
        result.validate_is_not_past_password();
        result
    }
    pub fn hash(&self) -> usize {
        // Really bad hash function
        // placeholder for illustration
        self.user.len() + 3 * self.pass.len() + 7 * self.salt.len()
    }
    // other functionality

    // Validate password
    fn validate_is_good(&self) {
        if self.pass.is_empty() {
            panic!("Empty password is not secure!");
        } else if self.pass.len() < MIN_PASS_LEN {
            panic!(
                "Password is too short: must be at least {} chars",
                MIN_PASS_LEN
            );
        }
        // password should contain at least one number
        fn is_number(ch: char) -> bool {
            ch.is_ascii_digit()
        }
        assert!(self.pass.chars().any(is_number));
        fn is_special(ch: char) -> bool {
            ch.is_ascii_punctuation()
        }
        assert!(self.pass.chars().any(is_special));

        // Make sure the password was not the same as username
        if self.pass == self.user {
            panic!("Password should not be same as username!");
        }

        // OK
    }

    fn validate_is_not_past_password(&self) {
        // get past pass hashes from a file
        // do some basic file handling
        let past_hashes = file_to_vec("PAST_HASH_FILE");
        for &hash in &past_hashes {
            if self.hash() == hash {
                panic!("Bad password: same as past hash!");
            }
        }
    }
}

// Function to get lines from a file
// Note: not recommended, better file handling in result.rs
pub fn file_to_vec(filepath: &str) -> Vec<usize> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    // Parses each line as an integer, collects them into a vec
    // NOTE: the use of .unwrap() twice!
    reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect()
}

/*
    DISCUSSION:
    Problems with panic?

    The above development "works", but has become quite unweidly!
    Simply initializing a password object now:
    - can fail in at least 8 different ways
    - each way is a different line in the source code / reason
    - two of the ways are .unwrap() so they don't leave a good error message
    - interacts with the file system, which may be unclear from the interface

    None of this is exposed in the interface! The library user just sees this:

        pub fn new_password(user: &str, pass: &str, salt: usize) -> Self

    This is the motivation for more robust error handling, which is primarily
    done with the Result type.
*/

/*
    INTERLUDE: Debugging

    I use a lot of debug_assert!()
    (assertion valid only in debug mode, not in --release)

    I use println!(), unit testing, dbg!()

        dbg!(filepath)
        ^^ prints out debug info about an object

    There are better / feature-rich debuggers though
    If you use (or can install on the side) VSCode there is a generally
    well-regarded debugger:

    (closest analogue to GDB)
    https://stackoverflow.com/questions/37586216/step-by-step-interactive-debugger-for-rust

    Note that interpreted languages by nature (Python, etc.) always have better debugging
    support

*/
