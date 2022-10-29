use std::io;
use std::collections;

mod hello_world_but_in_another_file;
// Use an alias because the name is really long.
use hello_world_but_in_another_file::run_self as hwbiaf;
mod guess_the_number;

type FunctionMap = collections::HashMap<String, fn()>;

// TODO(JBierenbroodspot): Remove redundant Option.
/// Tries to get a key value from a `FunctionMap`.
/// 
/// This function takes a borrowed `str` and tries to get the value from a 
/// `FunctionMap`.
/// 
/// # Arguments
/// 
/// * `key` - A borrowed `str` that will be used as a key to find a function in
///           a map.
/// * `function_map` - A `HashMap` where `String` keys are paired with functions
///                    as values.
/// 
/// # Return value
/// 
/// Returns a borrowed function if the key exists within the map, otherwise 
/// `None` is returned.
fn get_from_fn_map<'a>(key: &str, 
                       // `'a` is a lifetime annotation. In this case it shows
                       // that `&'a fn()` is borrowed from `function_map` and
                       // not `key`.
                       function_map: &'a FunctionMap) -> Option<&'a fn()> {  
    return function_map.get(key);
}

fn main() -> io::Result<()> {
    let stdin: io::Stdin = io::stdin();

    let mut user_input: String;
    let mut app: Option<&fn()> = None;

    let mut function_map: FunctionMap = collections::HashMap::new();
    function_map.insert("1".to_string(), hwbiaf);
    function_map.insert("2".to_string(), guess_the_number::run_self);

    
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
                   .and(get_from_fn_map(&user_input, &function_map));
    }

    if let Some(some_app) = app {
        (some_app)();
    }

    Ok(())
}
