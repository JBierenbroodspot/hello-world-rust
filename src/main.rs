use std::io;

mod hello_world_but_in_another_file;
mod guess_the_number;

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
fn get_selection(selection: Option<&str>) -> Option<fn()> {
    // TODO(JBierenbroodspot): Find a way to store which module to use so it can
    //                         be ran outside of the main loop.
    // String returned by `read_line()` include a trailing newline character.
    match selection.map(|s| s.trim()) {
        Some("1") => {return Some(hello_world_but_in_another_file::run_self);},
        Some("2") => {return Some(guess_the_number::run_self);},
        _   => return None
    }
}

fn main() -> io::Result<()> {
    let stdin: io::Stdin = io::stdin();

    let mut user_input: String;
    let mut app: Option<fn()> = None;
    
    while app.is_none() {
        user_input = String::new();

        // `stdin.read_line` returns a `io::Result` which is comparable to 
        // Haskell's `Maybe` monad. A `Result` contains either a success value
        // or an error value.
        app = stdin.read_line(&mut user_input)
                    // Turn previous type into an option so it can be compared
                    // to the return type of `get_selection`.
                   .ok()
                   // This will only be evaluated if the previous statement is
                   // of type `Some(T)`. Otherwise it id ignored and the value
                   // of the previous statement will be returned, which will be
                   // `None`.
                   .and(get_selection(Some(&user_input)));
    }

    if let Some(some_app) = app {
        (some_app)();
    }

    Ok(())
}
