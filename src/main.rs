use std::io;

pub mod hello_world_but_in_another_file;
pub mod guess_the_number;

/// Takes a `&str` and runs a function based on specific matching values.
/// 
/// This function will match a borrowed `str` to hardcoded options and run a 
/// function if there's a match.
/// 
/// # Arguments
/// 
/// * `selection` - A borrowed string to match against the existing options.
/// 
/// # Return value
/// 
/// If a function is successfully executed `true` is returned, otherwise `false`
/// is returned.
fn run_selection(selection: &str) -> bool {

    // TODO(JBierenbroodspot): Find a way to store which module to use so it can
    //                         be ran outside of the main loop.
    // String returned by `read_line()` include a trailing newline character.
    match selection.trim() {
        "1" => hello_world_but_in_another_file::run_self(),
        _   => return false
    }

    return true;
}

fn main() -> io::Result<()> {
    let stdin: io::Stdin = io::stdin();

    let mut stop: bool = false;
    let mut user_input: String;
    
    while stop != true {
        user_input = String::new();

        // `stdin.read_line` returns a `io::Result` which is comparable to 
        // Haskell's `Maybe` monad. A `Result` contains either a success value
        // or an error value.
        match stdin.read_line(&mut user_input) {
            Err(_) => stop = true,
            Ok(_) => stop = run_selection(&user_input)
        }
    }

    Ok(())
}
