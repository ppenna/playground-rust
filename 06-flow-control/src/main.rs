// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

///
/// This tutorial introduces you to the following concepts in Rust:
///  - If Then Else
///  - While
///  - Loop
///  - For
///  - Match
///
fn main() {
    // TODO: Activity 1
    // - Checkout the 'if_then_else_statement' example.
    // - Build and run it using cargo.

    // TODO: Activity 2
    // - Checkout the 'while_statement' example.
    // - Build and run it using cargo.

    // TODO: Activity 3
    // - Checkout the 'loop_statement' example.
    // - Build and run it using cargo.

    // TODO: Activity 4
    // - Checkout the 'for_statement' example.
    // - Build and run it using cargo.

    // TODO: Activity 5
    // - Checkout the 'match_statement' example.
    // - Build and run it using cargo.
}

//==============================================================================
// If-Then-Else
//==============================================================================

#[test]
fn if_then_else_statement() {
    let a: usize = 23;
    let b: usize = 7;

    // If-Else-If.
    if a < b {
        println!("{} < {}", a, b);
    } else if a > b {
        println!("{} > {}", a, b);
    } else {
        println!("{} == {}", a, b);
    }

    // Let-if.
    let smallest: usize = if a < b { a } else { b };
    println!("smallest: {}", smallest);
}

//==============================================================================
// While
//==============================================================================

#[test]
fn while_statement() {
    let mut x: usize = 10;

    // While.
    while x > 0 {
        println!("i: {}", x);
        x -= 1;
    }
}

//==============================================================================
// Loop
//==============================================================================

#[test]
fn loop_statement() {
    let mut x: usize = 10;

    // Loop.
    loop {
        if x == 0 {
            break;
        }

        x -= 1;

        if (x % 2) == 0 {
            continue;
        }

        println!("i: {}", x);
    }
}

//==============================================================================
// For
//==============================================================================

#[test]
fn for_statement() {
    let x: usize = 10;

    // For (exclusive).
    for i in 0..x {
        println!("i: {}", i);
    }

    // For (inclusive).
    for i in 0..=x {
        println!("i: {}", i);
    }

    // For each.
    for w in vec!["operating", "systems", "are", "awesome"] {
        println!("w: {}", w);
    }
}

//==============================================================================
// Match
//==============================================================================

#[test]
fn match_statement() {
    let x: usize = 6;
    let n: usize = 10;

    // Match.
    match x {
        0 => println!("x == 0"),
        1 => println!("x == 1"),
        2 => println!("x == 2"),
        3 => println!("x == 3"),
        4 => println!("x == 4"),
        5 => println!("x == 5"),
        x if x < n => println!("x < {}", n),
        _ => println!("x >= {}", n),
    }
}
