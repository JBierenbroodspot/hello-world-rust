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
fn get_selection(selection: &str) -> Option<fn()> {
    // TODO(JBierenbroodspot): Find a way to store which module to use so it can
    //                         be ran outside of the main loop.
    // String returned by `read_line()` include a trailing newline character.
    match selection.trim() {
        "1" => {return Some(hello_world_but_in_another_file::run_self);},
        "2" => {return Some(guess_the_number::run_self);},
        _   => return None
    }
}

fn main() -> io::Result<()> {
    let stdin: io::Stdin = io::stdin();

    let mut stop: bool = false;
    let mut user_input: String;
    let mut app: Option<fn()> = None;
    
    while stop != true {
        user_input = String::new();

        // `stdin.read_line` returns a `io::Result` which is comparable to 
        // Haskell's `Maybe` monad. A `Result` contains either a success value
        // or an error value.
        match stdin.read_line(&mut user_input) {
            Err(_) => stop = false,
            Ok(_) => {
                app = get_selection(&user_input); 
                stop = !app.is_none();
            }
        }
    }

    if let Some(some_app) = app {
        (some_app)();
    }

    Ok(())
}
