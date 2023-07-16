// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

///
/// This tutorial introduces you to the following concepts in Rust:
///  - Traits
///  - Trait Implementation
///  - Trait Bounds
///  - Derive attributes
///
fn main() {
    let mut lilo: Cat = Animal::new(String::from("Lilo"));
    let mut luna: Cat = Animal::new(String::from("Luna"));

    lilo.talk();
    lilo.eat();
    lilo.talk();

    luna.talk();
    feed(&mut luna);
    luna.talk();

    // Use derive Debug trait.
    println!("{:?}", lilo);
    println!("{:?}", luna);

    // TODO: Activity 1
    // - Write a 'Dog' structure that implements the 'Animal' trait.
    //  - Override the default 'talk()' in your implementation.
}

//==============================================================================
// Trait
//==============================================================================

/// A trait - a collection of methods defined for an unknown type.
trait Animal {
    // Associated function signature.
    fn new(name: String) -> Self;

    // Method signatures.
    fn name(&self) -> String;
    fn noise(&self) -> String;
    fn eat(&mut self);

    // Default method definitions. They can be overridden.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }
}

//==============================================================================
// Trait Implementation
//==============================================================================

/// A structure.
/// The 'derive' attribute automatically creates the implementation for the
/// 'Debug' trait.
#[derive(Debug)]
struct Cat {
    name: String,
    happy: bool,
}

/// Associated function for structure.
impl Cat {
    fn is_happy(&self) -> bool {
        self.happy
    }
}

/// Trait implementation.
impl Animal for Cat {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: String) -> Cat {
        Cat { name, happy: false }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn noise(&self) -> String {
        if self.is_happy() {
            return String::from("meow!");
        } else {
            return String::from("meow! meow! meow! meow!");
        }
    }

    fn eat(&mut self) {
        self.happy = true;
    }
}

//==============================================================================
// Trait Bounds
//==============================================================================

/// A generic function that operates on any `Animal`.
fn feed<T: Animal>(animal: &mut T) {
    println!("Feeding {}... ", animal.name());
    animal.eat();
}
