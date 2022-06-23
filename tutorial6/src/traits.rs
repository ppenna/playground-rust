/*
    Traits in Rust

    A running example using familiar concepts: address book
    - Database of information about people
    - Fields in the database: struct
    - Whole database: HashMap
*/

use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::ops::{Deref, Index};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Person {
    name: String,
    age: u8,
    phone: [u8; 10], // fixed array of 10 digits
    favorite_color: String,
}

pub struct AddressBook {
    // Note: struct fields
    // should not usually be public, normally would want to hide implementation
    // details in the API for your data structure
    by_name: HashMap<String, Person>,
    by_age: HashMap<u8, Vec<Person>>,
}
impl AddressBook {
    pub fn new() -> Self {
        Self { by_name: HashMap::new(), by_age: HashMap::new() }
    }
    pub fn add_person(&mut self, person: Person) {
        self.by_name.insert(person.name.clone(), person.clone());
        // Should be using the entry API
        // But I'm just illustrating here
        self.by_age.insert(person.age, Vec::new());
        self.by_age.get_mut(&person.age).unwrap().push(person);
    }
}

/*
    ***** QUIZ *****

    1. What will fail to compile the above code?
    A: we needed to derive Clone and use .clone()

    2. What would happen now if we write a test and do
        assert_eq!(Person1, Person2)
       ?
    A: We needed to derive Debug and PartialEq
*/

#[test]
fn test_assert_eq_person() {
    let person1 = Person {
        name: "caleb".to_owned(),
        age: 26,
        phone: [5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        favorite_color: "Purple".to_owned(),
    };
    let person2 = person1.clone();
    assert_eq!(person1, person2);
}

/*
    ***** Part 1 *****
    Implementing a trait for a type
*/

/*
    Clone and Copy

    Clone is for .clone() to copy all your data from one place in memory
    to another.

    Copy is for automatically cloning on function calls and other places
    so you don't have to worry about it.

    Q: Should we implement Copy for Person?
    A: Probably no, Person has String fields which are dynamically growable,
       so we don't really want to automatically copy large chunks of memory
       without making that explicit and making the user think about it.
       Copy is mainly used for integer, char, etc. data types.

    Q: Should we implement Copy for AddressBook?
    A: Even worse, definitely no.
*/

/*
    So far: to implement a trait, we have seen the #[derive(...)]
    syntax.

    What if we want a different implementation than the default provided
    by #[derive(...)]?

    #[derive(...)] is just magic that calls a macro to generate some code
    to implement your trait automatically in some canonical or "obvious" way.
    If you want something less obvious or different than the default,
    you can write that code yourself.
*/

// Let's take Eq as an example, what would be the automatically generated Eq
// code?
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.age == other.age
            && self.phone == other.phone
            && self.favorite_color == other.favorite_color
    }
}

// But maybe we want to say two people are equal if they have the same name?
// impl PartialEq for Person {
//     fn eq(&self, other: &Self) -> bool {
//         self.name == other.name
//     }
// }

// In general, the way you implement a trait:
// - First you look up the documentation or the source code for the trait
//   which will have a list of methods that you need to implement.
//   For example, Clone requires that you implement
//       fn clone(&self) -> Self
//   PartialEq requries that you implement
//       fn eq(&self, other: &Self) -> bool
//   and so on.
//   Some traits have more than just one function.

/*
    ***** Part 2 *****
    More standard library traits
*/

/*
    Debug and Display
    The displaying traits in Rust
        Debug: "{:?}"
        Display: "{}"
    The reason they're different? The idea is to keep
    straight different purposes: Debug should be
    "print this out for a developer to take a look at and understand"
    Display should be "print this out to the end user of the application"
*/

// Let's try implementing display for our Person type
// (we could also do it for AddressBook)
// Brief note:
//    Display and Debug make use of certain types specific to the fmt
//    module, so you need fmt::Formatter and fmt::Result in particular.
//    you can think of fmt::Result as just a customization of Result<(), String>
impl Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Person with name {}, age {}, other details omitted",
            self.name, self.age,
        )
    }
}

#[test]
fn test_display_person() {
    let caleb = Person {
        name: "caleb".to_owned(),
        age: 26,
        phone: [5, 5, 5, 5, 5, 5, 5, 5, 5, 5],
        favorite_color: "Purple".to_owned(),
    };
    println!("{}", caleb);
    // Useful trick:
    // Test output is suppressed for passing tests :(
    // if you want to see the output from a test,
    // try causing the test to fail.
    // assert!(false);
}

// Example from Piazza Q
// If you want to re-implement Display for a type that already exists,
// you need to make a wrapper around that type, like this:
struct Potato(Vec<i32>);

impl Display for Potato {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Potato({:?})", self.0)?;
        Ok(())
    }
}

/*
    From and Into

    Exactly what it sounds like: going from one type to another.

    Important note: If you implement From, then Into is derived
*/

const DEFAULT_PHONE: [u8; 10] = [5, 5, 5, 5, 5, 5, 5, 5, 5, 5];

impl From<(String, u8)> for Person {
    // Convert a String to a Person
    fn from((name, age): (String, u8)) -> Self {
        Person {
            name,
            age,
            phone: DEFAULT_PHONE,
            favorite_color: "Unknown".to_string(),
        }
    }
}

#[test]
fn test_from_into() {
    let caleb = Person::from(("caleb".to_owned(), 26));
    assert_eq!(caleb.phone, DEFAULT_PHONE);

    // Into is automatically implemented by the std library
    // if you implement From
    let caleb2: Person = ("caleb".to_owned(), 26).into();
    assert_eq!(caleb, caleb2);

    // Note: from is an associated function (doesn't take self),
    // so called with ::from
    // into is a method, so called with .into().
}

/*
    Default

    Rust uses Default for objects which can be safely initialized automatically
    to some default value.

    If you implement Default, you are saying "it's fine to not initialize me, I
    will just be the default value in that case"

    Good example of NOT deriving a trait: by not implementing Default
    for Person, we indicate to users of our data structure or library
    that they should always initialize a Person. ==> compiler error
    if you try to use Person with an API that requires Default.

    But Default does make sense for AddressBook, because you can have
    an empty address book and that's fine.
*/

impl Default for AddressBook {
    fn default() -> Self {
        Self { by_name: Default::default(), by_age: Default::default() }
        // ^^ HashMap implements Default!
    }
}

#[test]
fn test_address_book_default() {
    // People usually do this
    #[allow(unused_variables)]
    let a1: AddressBook = Default::default();
    // This should work too
    #[allow(unused_variables)]
    let a2 = AddressBook::default();
    // Could do this, need to first implement Debug and PartialEq, just
    // like we did for Person
    // assert_eq!(a1, a2);
}

/*
    FromStr: can-fail conversion

    for "Parsing" -- any time you want to parse your data from a String,
    you want to implement FromStr

    Q: Why don't I just implement From for String?

    impl From<String> for Person {
        ...
    }

    Parsing can fail!
    We would prefer not to panic on failure, and to return
    a Result error.
*/

impl FromStr for Person {
    // New we haven't seen -- specify a type as part of the trait
    // Called an "associated type"
    // It's actually internally similar to something we have seen:
    // recall From syntax:
    // From<(String, u8)>
    // (String, u8) is basically the same thing -- a type that you
    // specify along with implementing the trait.
    type Err = String;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        // Complex parsing logic here
        // Parsing can sometimes be annoying
        // &str API has a bunch of useful functions, particularly
        // matching patterns
        // https://doc.rust-lang.org/std/primitive.str.html
        // Rough pseudocode:
        // - Check if the string starts with 'Person'
        // - Then split the remainder of the string by a separator
        //   character ',', by calling .split()
        // - For each part, try to parse it as the corresponding
        //   field of Person
        // - For each line that fails, use the ? operator:
        //       let name = split_parts[2].parse()?;
        //   That forwards the error case to return from the function
        //   immediately.
        // In the end you would end up with something where you
        // can call "Person(caleb, 26, ...)".parse()
        // to get a person object.
        unimplemented!()
    }
}

/*
    Common operations:
    std::ops::{Index, IndexMut, Add, Mul}

    Index, IndexMut, Add, Mul

    These correspond to specific syntax:
    Index, IndexMut:
        []
    Add:
        +
    Mul:
        *
    Others, e.g. AddAssign for +=

    For our AddressBook type, of these,
    it seems like Index and IndexMut make sense, not necessarily
    the others
*/

// Maybe we want to directly access a Person entry by name
impl Index<&str> for AddressBook {
    type Output = Person;
    fn index(&self, idx: &str) -> &Person {
        &self.by_name[idx]
    }
}

// Now if I have an AddressBook a
// I can use the syntax println!("{}", a["caleb"])
// rather than using another method.

/*
    *******************************
    End of part 1 -- will continue next time.
    *******************************
*/

/*
    Others:
    - Deref
    - AsRef
    - Borrow

      Trait to do automatic conversion when that is more
      convenient than manually converting between types.
*/

// Maybe you want to leave the possibility open to
// change what a PhoneNumber is and how it's implemented,
// so you decided to abstract it as a type.

pub struct PhoneNumber([u8; 10]);

pub struct Person2 {
    pub name: String,
    pub age: u8,
    pub phone: PhoneNumber,
    pub favorite_color: String,
}

// Q: What might go wrong if I have an existing code base
// (say, large industry-scale project) and I decided to
// add this PhoneNumber type?

// There might be multiple possible answers here, but one
// thing that would definitely go wrong is that everywhere
// phone is used, in particular wherever Person2 is used,
// you have to update the code.
//      ==> downsides: potentially invasive/breaking changes
//      ==> lose readability

impl Person2 {
    pub fn existing_method(&self) {
        // Existing code: errors because self.phone is now
        // a PhoneNumber instead of a [u8; 10]
        // let area_code = self.phone[0..3];
        let area_code = &self.phone.0[0..3];
        println!("Area code: {:?}", area_code);
    }
}

// Idea: **treat** PhoneNumber as "automatically convertible" to [u8; 10].
// Writing .0 everywhere is annoying sometimes
// Maybe it is safe to treat these as essentially interchangeable.

// In Rust, there are a few traits to accomplish this, but the most
// simple and useful one is to convert on dereference, via either AsRef
// or Deref
// Let's show Deref here (more automatic)

impl Deref for PhoneNumber {
    type Target = [u8; 10];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
    Now magically the existing code should just convert
    implicitly and not need the .0
*/
impl Person2 {
    pub fn existing_method2(&self) {
        // Existing code: errors because self.phone is now
        // a PhoneNumber instead of a [u8; 10]
        let area_code = &self.phone[0..3];
        println!("Area code: {:?}", area_code);
    }
}

/*
    A couple more traits before we move on to iplementing our own.

    - Read / Write

      For objects that can be read or written to.

    println!("{}", ...)
    write!(f, "{}", ...)
    ^ for f implementing the Write trait.

    Q: Why is this useful?

    A: Very different objects are writable.
    - a terminal interface
    - standard output
    - a network connection
    - a internal buffer
    - A file

    Recall: Traits define shared behavior across different types!

    Similarly Read is for stdin-like buffers, input files, etc.
*/

/*
    Iterator
*/

pub fn iterator_example() {
    for x in 0..10 {
        println!("{}", x);
    }
    // Internally: for x in I means that I is a type implementing
    // the Iterator trait.

    // Most collections offer a way of getting an iterator from
    // the object; where "iterator" means something that implements
    // the Iterator trait.

    let v = vec![1, 2, 3];
    for x in v.iter() {
        println!("{}", x);
    }

    let h: HashMap<usize, usize> = HashMap::new();
    for x in h.iter() {
        println!("{:?}", x);
    }
}

/*
    Not technically in the standard library, but so widespread and
    idiomatic that they are standard:

    StructOpt
    https://crates.io/crates/structopt
    - StructOpt trait: if you want to parse your data from the command line

    Serde
    https://crates.io/crates/serde
    - Serialize and Deserialize traits

    Like with StructOpt,
    You can usually just #[derive(Serialize, Deserialize)]
    and it "just works".
*/

/*
    ***** Part 3 *****
    Defining your own traits

    Traits Define Shared Behavior Across Types.

    In implementing software we often find that similar behavior
    recurs across types.

    For our AddressBook example, suppose that we want to
    print out a "summary view" of the AddressBook to the user,
    e.g. the first 10 people in it by name and age.
*/
impl AddressBook {
    pub fn print_summary_first10_entries(&self) {
        for _entry in self.by_name.iter().take(10) {
            // Take: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.take
            // **some logic to print out a summarized entry here. **
        }
    }
}

/*
    Now we take a look at the types inside our AddressBook:
    we have:
        - Person
        - Vec<Person>
        - Fields inside Person, like [u8; 10] and String

    Q: For our AddressBook summary, what behavior would we like the above types
    to satisfy?

    Having identified the common behavior, let's write a trait.
*/

trait Summary {
    // Documentation: this trait encapsulates the behavior of printing
    // a short summary of a datatype.
    // (i.e. abbreviated)

    // What behavior defines this trait?

    fn short_summary(&self) -> String;
    fn long_summary(&self) -> String;
    // Summarize the type in 'lines' lines or fewer.
    fn summary_in_lines(&self, lines: usize) -> String;
}

// Now we can implement Summary for different types.

impl Summary for PhoneNumber {
    // phone numbers are already short.
    fn short_summary(&self) -> String {
        // In the context of format! apparently Deref
        // doesn't kick in
        format!("{:?}", &self.0)
    }
    fn long_summary(&self) -> String {
        self.short_summary()
    }
    fn summary_in_lines(&self, lines: usize) -> String {
        if lines > 0 {
            self.short_summary()
        } else {
            "".to_string()
        }
    }
}

impl Summary for Person2 {
    fn short_summary(&self) -> String {
        format!("Person: {}, age {}", self.name, self.age)
    }
    fn long_summary(&self) -> String {
        format!(
            "Person: {}, age {}, phone {}",
            self.name,
            self.age,
            self.phone.short_summary(),
        )
    }
    fn summary_in_lines(&self, _lines: usize) -> String {
        unimplemented!()
    }
}

// Our AddressBook has a Vec<Person> in it
impl Summary for Vec<Person2> {
    fn short_summary(&self) -> String {
        unimplemented!()
    }
    fn long_summary(&self) -> String {
        unimplemented!()
    }
    fn summary_in_lines(&self, lines: usize) -> String {
        // Iterating over the first 'lines' people
        let mut result = String::new();
        for item in self.iter().take(lines) {
            result += &item.summary_in_lines(1);
        }
        result
    }
}

// And finally I could then implement Summary for AddressBook
impl Summary for AddressBook {
    fn short_summary(&self) -> String {
        unimplemented!()
    }
    fn long_summary(&self) -> String {
        unimplemented!()
    }
    fn summary_in_lines(&self, _lines: usize) -> String {
        unimplemented!()
    }
}

/*
    One last thing about implementing your own traits:
    Sometimes you want do define derived functionality from
    the implemented shared functionality.

    For our Summary trait, maybe we want
    a summary_in_10_lines function. What should we do?
*/

trait Summary2 {
    fn short_summary(&self) -> String;
    fn long_summary(&self) -> String;
    // Summarize the type in 'lines' lines or fewer.
    fn summary_in_lines(&self, lines: usize) -> String;

    // Derived function!
    fn summary_in_10_lines(&self) -> String {
        self.summary_in_lines(10)
    }

    // The above provides a default implementation, so no type
    // implementing the trait has to be modified to add summary_in_10_lines.
}
