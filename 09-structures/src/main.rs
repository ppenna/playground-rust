// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

// Suppress some warnings.
#[allow(dead_code, unused_variables)]

///
/// This tutorial introduces you to the following concepts in Rust:
///  - Unit Structures
///  - Tuple Structures
///  - Structures
///  - Compound Structures
///
fn main() {
    // Instantiates a color.
    let black: Color = Color(0, 0, 0);

    // Instantiate two points.
    let p1: Point = Point::new(0.0, 0.0);
    let p2: Point = Point::new(1.0, 1.0);

    p1.print();
    p2.print();

    // TODO: Activity 1
    // Write associated functions for the 'Rectangle' structure.
    // It should have the following methods:
    //  - new(): constructs a new rectangle.
    //  - set_color(): changes the color or the rectangle.
    //  - width(): returns the width of the rectangle.
    //  - height(): returns the height of the rectangle.
    //  - area(): returns the area of the rectangle.
}
//==============================================================================
// A Unit Structure
//==============================================================================

/// A unit structure.
/// Unit structures carry no data, and are used in other language constructs
/// (see thee tutorial in Enumerations).
#[allow(dead_code)]
struct Unit;

//==============================================================================
// Tuple Structures
//==============================================================================

/// A tuple structure.
/// Tuple structures are structures in which its members do not have a name.
/// They are useful to wrap around to data types and provide safety.
struct Color(u8, u8, u8);

//==============================================================================
// Structures
//==============================================================================

/// A structure
struct Point {
    x: f32,
    y: f32,
}

/// These are the associated functions for `Point`.
/// Associated functions are functions that are associated with a structure.
impl Point {
    /// Creates a new `Point`.
    /// Unlike other languages, in Rust there is no special constructor function
    /// and you can name the function that does this job the way you prefer.
    fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    /// Prints a point.
    /// This function operates on a immutable `Point`.
    fn print(&self) {
        println!("({}, {})", self.x, self.y);
    }
}

//==============================================================================
// Compound Structures
//==============================================================================

/// A compound structure.
#[allow(dead_code)]
struct Rectangle {
    color: Color,
    top_left: Point,
    bottom_right: Point,
}
