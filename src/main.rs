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
            Ok(_) => println!("{}", user_input)
        }
    }

    Ok(())
}
