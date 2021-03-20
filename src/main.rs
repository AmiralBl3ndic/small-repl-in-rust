mod symbol;

use std::io::{self, Write, stdin, stdout};
use symbol::parser::number::{Number, parse_number};

fn main() -> Result<(), io::Error> {
    let stdin = stdin();
    let mut stdout = stdout();

    print!("Hello, world!\n> ");

    stdout.flush()?;

    let mut user_input = String::new();

    if let Err(e) = stdin.read_line(&mut user_input) {
        panic!("Unable to read user input: {}", e);
    } else {
        println!("You typed: {}", user_input);

        if let (Ok(_), _) = parse_number(user_input.clone(), 2) {
            println!("Done");
        } else {
            panic!("Something went wrong");
        }

        user_input.clear();
    }

    let x: Number = Number::Int(42isize);

    println!("{:#?}", x);

    Ok(())
}
