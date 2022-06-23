use std::fmt::Display;
use std::fs::File;
use std::hash::Hash;
use std::io::Result;
use std::ops::Deref;
use std::path::Path;

// Drop
// Frees up system resources.
// Question: What kind of system resources do we know?
trait Drop {
    fn drop(&mut self);
}

// Rust handles dropping values automatically.
// We can implement drop to have our own dropping behavior.
// Implicitly called when the value goes out of scope

// fields in a struct/enum are still dropped recusively.

// Only needed when we have some resource that we want freed.

// Sized
// This is a marker trait: no semantic use. Merely tells rust whether a type has a constant
// size at compile time.
// Examples: str and [T]
// Size is automatically implemented for types. Cannot be implemented by hand.

// Size is implicitly used in trait bounds. If you want to allow unsized data, you must
// opt-in with the T: ?Sized syntax, takes either.

// Clone
// Implements clone and clone from method.
trait Clone: Sized {
    fn clone(&self) -> Self;
    fn clone_from(&mut self, source: &Self) {
        *self = source.clone()
    }
}

// Make a copy!
// Clones can be expensive, e.g. copy an array.

// Most types in std implement clone. Clone must succeed, so cannot clone
// on things that can fail, e.g. clone file descriptor.

// Copy
// Another marker trait.
// Byte by byte copy, must be cheap and always succeed. So cannot deep copy types,
// or resources, instead use clone for that.

// Question: Cannot implement copy if type already implements drop, why?

// Deref and DerefMuf
// trait Deref {
//     type Target: ?Sized;
//     fn deref(&self) -> &Self::Target; // implicit lifetime
// }

// trait DerefMut: Deref { // "extends"
//     fn deref_mut(&mut self) -> &mut Self::Target;
// }

// "Just works"
pub fn foo() {
    let boxed = Box::new("hello world!");
    let boxed2 = Box::new("hello world!".to_string());
    boxed.split(' '); // the methods of str work on boxed thanks to deref traits.
    boxed2.split(' '); // two derefs!
}

// Additionally, these traits allow some automatic conversion between types:
pub fn foo2(_our_str: &str) {
    unimplemented!();
}

pub fn example1() {
    let string = "hello".to_string();
    foo2(&string); // How is &String converted to &str?
                   // Check out String's trait implementation for str:
                   // https://doc.rust-lang.org/std/string/struct.String.html
}

// Straight from the book:
// "If inserting a deref call would prevent a type mismatch, Rust inserts one for you."
// Called "deref coercions"

// Notice many "String" methods are not implemented for String at all! Instead they're
// "inhereted" from str via the deref trait.

// Derefs allow us to "wrap" types, while allowing us to work with the types as if they
// were unwrapped. These new types "own" the data below. Examples:
// A string "wraps" a   str
// A vector "wraps" a   [T]
// A Box    "wraps" any T
// A Rc     "wraps" any T
// etc...

// From Book:
// "Rust applies them to resolve type conflicts, but not satisfy bounds on type variable"

struct StrWrapper<'a> {
    wrapped_str: &'a str,
}

impl<'a> Deref for StrWrapper<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        self.wrapped_str
    }
}

pub fn example2() {
    let s = StrWrapper { wrapped_str: "hello" };
    foo2(&s);

    // print_it(& s);
    // print_it_gen(& s);
    // print_it_gen(& s as &str);
}

pub fn print_it(my_str: &str) {
    println!("{}", my_str);
}

pub fn print_it_gen<T: Display>(display: T) {
    println!("{}", display);
}

// Default
// Allows you to specify the default value for some types (when it makes sense)

// AsRef and AsMut
pub trait MyAsRef<T: ?Sized> {
    fn as_ref(&self) -> &T;
}

// You can borrow a T from your type efficiently.
// Examples:
// impl<T> AsRef<[T]> for Vec<T>
// impl AsRef<Path> for str
// impl AsRef<Path> for String
// impl AsRef<Path> for PathBuf

// notice no &str
// Notice these "impl" are read kinda backwards, that is the type we are allowing to
// convert to Path is on the right side, Path is on the left side so: Path <- T

// Example:
pub fn open<P: AsRef<Path>>(_path: P) -> Result<File> {
    unimplemented!()
}

pub fn use_open() {
    open("./myFile.txt").unwrap(); // type &str
}
// So why does this work if there is no AsRef for &str?
// Is it because of Deref? No! Remember Deref does not work to satify type variable trait
// bounds...

// Instead:
impl<'a, T, U> MyAsRef<U> for &'a T
where
    T: AsRef<U>,
    T: ?Sized,
    U: ?Sized,
{
    fn as_ref(&self) -> &U {
        (*self).as_ref()
    }
}

// So think about &str, here &str is &'a T => str ~ T
// In our Path example, U ~ Path so we have:
// impl AsRef<Path> for &str

// Important thing for me to remember: Given some type T, wlog say i32,
// *i32 or &i32 are not slightl variants of the same type, they're truly new types,
// which can implement their own traits, methods, etc.
// This is why I dislike the C/C++ notation:
// int *x = 5;

// AsMut: Mutable version.

// Borrow and BorrowMut
// Similar to AsRef

pub trait Borrow<Borrowed: ?Sized> {
    fn borrow(&self) -> &Borrowed;
}

// Difference:
// For impl Borrow<B> for T
// The value borrowed should hash (Hash) and compare (Eq, PartialOrd)
// in the same way as the &B

pub struct HashMap<K, V>
where
    K: Eq + Hash,
{
    // ... ignore these, needed to compile.
    pub k: K,
    pub v: V,
    // ...
}

impl<K, V> HashMap<K, V>
where
    K: Eq + Hash,
{
    pub fn get(&self, _key: K) -> Option<&V> {
        unimplemented!()
    }
}

// What's the problem with this implementation? Moves value.
// Instead: fn get(&self, key: &K) -> Option<&V>

// For String:
// fn get(&self, key: &String) -> Option<&String>

// What if all I have is a &str?
// I'm going to have to box this value up just to pass it.
// We kinda want:
// fn get(&self, key: &str) -> Option<&String>

// But how do you implement this generically for any type?
// https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get

// Paste definition over...

// pub fn get(&self, k: &str) -> Option<&String> where
// String: Borrow<str>,
// str: Hash + Eq,

// So does String impl Borrow<str> ?
// https://doc.rust-lang.org/std/string/struct.String.html#impl-Borrow%3Cstr%3E
// Eyyy!

// From and Into
pub trait Into<T>: Sized {
    fn into(self) -> T;
}
pub trait From<T>: Sized {
    fn from(_: T) -> Self;
}

// These don't look too fancy or special, or particularly powerful?

// For vector https://doc.rust-lang.org/std/vec/struct.Vec.html#deref-methods
// That's neat... but nothing we couldn't do with regular old'e function?

// But as seen on the last few examples, they're powerful when used as trait bounds
// on type varaibles for generic functions.

// Function nix::sys::wait::waitpid
// pub fn waitpid<P: Into<Option<Pid>>>(pid: P, options: Option<WaitPidFlag>) ->
// Result<WaitStatus> {
// unimplemented!()
// }

// So now wait pid works on types:
// let pid : Pid = ...;
// waitpid(pid, None);
// waitpid(None, None);
// waitpid(Some(pid), None);

// How is Pid converted to Some<Pid> ?

// Only need to implement one of them.

// ToOwned
// trait Clone: Sized {
// fn clone(&self) -> Self;
// ...
// }

// Bug what if we want to copy a &str or a &[T] ?
// Clone doesn't really work, it can only return something of the same type,
// for reference types like this, what won't work.

pub trait ToOwned {
    type Owned: Borrow<Self>;
    fn to_owned(&self) -> Self::Owned;
}

// By now you should have noticed sometimes we parameterize a trait using
// a type parameter, other times using associated types what's the difference?
// left to you, reader, to think about. (Explained in class)

// COW
// Let's you decide at runtime whether we should borrow or we should own a value.
pub enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized,
{
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}

// Why is cow useful?
//Checkout example from docs:
// https://doc.rust-lang.org/std/borrow/enum.Cow.html

// Show example from book.
