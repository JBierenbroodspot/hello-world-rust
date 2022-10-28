//! This module contains a function to print `Hello, world!` to the console and
//! a function to run said function.

/// Runs the module.
pub fn run_self() {
    print_hello_world();
}

/// Prints Hello, world! to console.
fn print_hello_world() {
    println!("Hello, world!");
}