use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Result;

/*
    Result<T, U>
    is kind of like Option<T>
    and is similar to an enum:

    enum Result {
        Ok(T)
        Err(U)
    }

    Example values of Result<usize, String>:
        Ok(3)
        Ok(4)
        Err("not an integer")
        Err("did not parse")
        ...

    If you're writing a function that might fail,
    instead of calling panic!() always consider
    returning a Result.
*/

pub fn read_whole_file(file: &str) -> String {
    // Example from the standard library 1: File::open
    // pub pub open<P: AsRef<Path>>(path: P) -> Result<File>
    let fd = File::open(file);
    let our_file = match fd {
        Ok(our_file) => our_file,
        Err(e) => panic!("Unable to open file: {}", e),
    };

    // Example from the standard library 2: BufRead .lines()
    let reader = BufReader::new(our_file);
    let mut string = String::new();
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => panic!("Failed to read line: {}", e),
        };

        string.push_str(&line);
    }

    string
}

/*
    Useful syntax to "propagate" results up from functions you call
    up to the caller:
        ?
*/

pub fn read_whole_file_decent(file: &str) -> Result<String> {
    let fd = File::open(file)?; // fd is a File, not a Result

    // Roughly equivalent to the following:
    // let fd = match File::open(file) {
    //     Ok(ok) => ok,
    //     Err(e) => return e,
    // }
    // better than .unwrap(), .expect(), panic, etc. if you aren't in a main()
    // function
    // or otherwise in a context where you just would rather panic.

    let reader = BufReader::new(fd);
    let mut string = String::new();
    for line in reader.lines() {
        // line is a result
        // Intuitively ? is how you get rid of a result.
        // Only works when the output type of my function is Result
        string.push_str(&line?);
    }

    Ok(string)
}

pub fn read_whole_file_good(file: &str) -> Result<String> {
    let mut string = String::new();
    let file = File::open(file)?;
    let mut reader = BufReader::new(file);
    reader.read_to_string(&mut string)?;
    Ok(string)
}

// Similarly:
// pub fn file_to_vec(filepath: &str) -> Result<Vec<usize>, String> {

// More general syntax:
// Result<Vec<usize>, String>
// Syntax specific to the std::io::Result object:
// Result<String>

// People used to write this kind of code.
pub fn our_main() {
    match run() {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}

pub fn run() -> Result<()> {
    read_whole_file_good("foo.txt")?;
    // ...
    read_whole_file_good("foo2.txt")?;

    Ok(())
}

// Newer versions of rust let you simply have main() return a result:
// This is if you want to be a stickler who doesn't believe in ever
// panicking or unwrap / expect / etc.
pub fn main() -> Result<()> {
    read_whole_file_good("foo.txt")?;
    // instead of read_whole_file_good("foo.txt").expect("unexpected thing happened")
    // ...
    read_whole_file_good("foo2.txt")?;

    Ok(())
}

// Handling errors of multiple error types.

// Fancy Box<dyn Error> type!
// dyn means dynamic -- means the type of the object is dynamic
// If you're familiar with Python, you could think of
// dyn X as Python "duck type"
//   - "if it quacks like a duck and walks like a duck it is a duck"

// E.g. when parsing integer as string, you get something like a ParseError
// error result type
// Returning Box<dyn Error> allows working both with the ParseError and any other
// errors that might be encountered, e.g. strings.

// Trait object can hold any type which implements some trait. For this,
// it is the Error trait.
pub fn f() -> Box<dyn Error> {
    unimplemented!();
}

pub type GenResult<T> = ::std::result::Result<T, Box<dyn Error>>;

// Dealing with errors that "can't" happen
// Call unwrap or expect on your Result value
