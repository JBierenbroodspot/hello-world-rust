//! This module is a game where the user is to guess a number generated by the
//! computer within a given amount of tries.

use std::io;
use rand::Rng;

/// Generates a random unsigned 8 bit integer.
fn generate_random_number() -> u8 {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    return rng.gen();
}

/// Compares two borrowed unsigned 8 bit integers.
/// 
/// This function takes an optional `u8` and a non-optional `u8` and compares 
/// them. 
/// 
/// # Arguments
/// 
/// * `guess` - A `u8` to be compared against `to_guess`.
/// * `to_guess` - A `u8` to have `guess` compared against.
/// 
/// # Return value
/// 
/// The return value is a tuple of two booleans, the left boolean stands for 
/// `guess < to_guess` and the right one stands for `guess > to_guess`. That
/// that if return value is `(true, false)` it means that `guess` is smaller 
/// than `to_guess`. If both are `true` it means something has gone wrong since
/// it is impossible for a value to be both above and below another. If both are
/// `false` it means the numbers are equal.
/// 
/// # Example
/// 
/// ```
/// let mut guess: Option<u8> = Some(50);
/// let mut to_guess: u8 = 60;
/// evaluate_guess(&guess, &to_guess); // -> (true, false)
/// 
/// guess = Some(100);
/// evaluate_guess(&guess, &to_guess); // -> (false, true)
/// 
/// guess = Some(60);
/// evaluate_guess(&guess, &to_guess); // -> (false, false)
/// 
/// guess = None;
/// evaluate_guess(&guess, &to_guess); // -> (true, true)
/// ```
fn evaluate_guess(guess: Option<u8>, to_guess: &u8) -> (bool, bool) {
    return guess.map(|num| (num < *to_guess, num > *to_guess))
                .or(Some((true, true)))
                // Won't panic because this will always evaluate to `Some()`.
                .unwrap();
}

// TODO(JBierenbroodspot): update documentation.
/// Converts string to a unsigned 8 bit integer.
/// 
/// # Arguments
/// 
/// * `guess` - A borrowed string, or `None`, that can be parsed into an 
///             unsigned 8 bit integer.
/// 
/// # Return value
/// 
/// If `guess` can be successfully parsed into a unsigned 8 bit integer then 
/// `Some(u8)` will be returned, otherwise `None` is returned.
fn guess_to_u8(guess: &str) -> Option<u8> {
    return guess.to_string().parse::<u8>().ok();
}

pub fn run_self() {
    let stdin: io::Stdin = io::stdin();
    let to_guess: u8 = generate_random_number();

    let mut stop: bool = false;
    let mut user_input: String = String::new();
    let mut result: (bool, bool);
    let mut guess: Option<u8>;
    
    while stop != true {
        user_input.clear();

        println!("Enter your guess ({} - {}):", u8::MIN, u8::MAX);

        guess = stdin.read_line(&mut user_input)
                     .ok()
                     .and(guess_to_u8(&(user_input.trim())));

        result = evaluate_guess(guess, &to_guess);

        println!("The result is {:?}", result);
        println!("The random number is {}", to_guess);
        stop = true;
    }
}