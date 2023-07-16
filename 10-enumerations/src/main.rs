// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

// Suppress some warnings.
#[allow(dead_code)]

///
/// This tutorial introduces you to the following concepts in Rust:
///  - Enumerations
///
fn main() {
    let pressed: WebEvent = WebEvent::KeyPress('x');
    let click: WebEvent = WebEvent::Click { x: 20, y: 80 };
    let load: WebEvent = WebEvent::PageLoad;
    let unload: WebEvent = WebEvent::PageUnload;

    inspect(pressed);
    inspect(click);
    inspect(load);
    inspect(unload);

    // TODO: Activity 1
    // - Write the following constructor-like associated functions:
    //  - 'click()': constructs a new 'WebEvent::Click'.
    //  - 'page_load()': constructs a new 'WebEvent::PageLoad'.
    //  - 'page_unload()': constructs a new 'WebEvent::PageUnload'.
    //  - 'key_press()': constructs a new 'WebEvent::KeyPress'.
    // - Replace how the 'WebEvent' enumeration is instantiated in the 'main()'
}

//==============================================================================
// Enumerations
//==============================================================================

/// An enumeration - a compound data type that enables you to enumerate its
/// possible values.
enum WebEvent {
    // An enumeration variant may either be a unit structure.
    PageLoad,
    PageUnload,
    // An enumeration variant may be a tuple-structure.
    KeyPress(char),
    // An enumeration variant may be a usual structure.
    Click { x: i64, y: i64 },
}

// Associated functions for the enumeration.
impl WebEvent {}

// A function that operates on an enumeration.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}
