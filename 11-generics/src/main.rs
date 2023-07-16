// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

///
/// This tutorial introduces you to the following concepts in Rust:
///  - Generic structures
///
fn main() {
    let p: Point<usize> = Point::new(10, 20);
    println!("p.x = {}, p.y = {}", p.x(), p.y());
    let p: Point<f64> = Point::new(10.0, 20.0);
    println!("p.x = {}, p.y = {}", p.x(), p.y());

    // TODO: Activity 1
    // - Write a generic structure and associated functions for a stack.
}

//==============================================================================
// Generic Structures
//==============================================================================

// A generic structure.
struct Point<T> {
    x: T,
    y: T,
}

// A generic implementation for the structure.
impl<T> Point<T> {
    // A generic method.
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    // A generic method.
    fn x(&self) -> &T {
        &self.x
    }

    // A generic method.
    fn y(&self) -> &T {
        &self.y
    }
}
