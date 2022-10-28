use std::io;

pub mod hello_world_but_in_another_file;

fn main() -> io::Result<()> {
    let mut stop: bool = false;
    let mut user_input: String;
    let mut stdin: io::Stdin;
    
    while stop != true {
        stdin = io::stdin();
        user_input = String::new();

        match stdin.read_line(&mut user_input) {
            Err(_) => stop = true,
            Ok(_) => stop = run_selection(&user_input)
        }
    }

    Ok(())
}

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

    // String returned by `read_line()` include a trailing newline character.
    match selection.trim() {
        "1" => hello_world_but_in_another_file::run_self(),
        _   => return false
    }

    return true;
}
