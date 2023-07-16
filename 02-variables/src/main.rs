// Copyright(C) 2023 Pedro Henrique Penna <pedrohenriquepenna@gmail.com>
// Licensed under the MIT license.

// Suppress some warnings.
#[allow(unused_variables, unused_mut, unused_assignments)]

///
/// This tutorial introduces you to the following concepts in Rust:
///  - Variables
///  - Mutability
///  - Scopes
///  - Shadowing
///
fn main() {
    // This is an immutable variable. You cannot change its value.
    // Rust variables are immutable by default.
    let imut_var = 2;
    println!("imut_var = {}", imut_var);

    // This is a mutable variable. You can change its value.
    // Note that rust variables are named using snake case.
    let mut mut_var = 32;
    mut_var = 64;
    println!("mut_var = {}", mut_var);

    // This is a block, which defines a new scope in Rust.
    {
        // This variable lives only on this scope.
        let short_lived_var = 4;
        println!("short_lived_var = {}", short_lived_var);

        // This is a shadowed variable.
        // It overrides the visibility of a previous variable with the same name.
        let imut_var = 3;
        println!("imut_var = {}", imut_var);
    }
    // End of the block.

    // TODO: Activity 1
    // - Uncomment the following print statement.
    // - Attempt to build and run this example.

    // println!("short_lived_var = {}", short_lived_var);

    println!("imut_var = {}", imut_var);
}
