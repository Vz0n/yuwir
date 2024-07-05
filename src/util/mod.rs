use std::io::{stdin, stdout, Write};
use std::str::FromStr;
use crate::exit;

pub fn read_line(buff: &mut String) -> &str{
    // First, clear buffer and flush stdout
    buff.clear();
    let _ = stdout().flush();

    // Now, read and write
    let _ = stdin().read_line(buff);
    buff.trim()
}

pub fn read_sized<T: FromStr>(input: &mut String) -> T{
    match read_line(input).parse::<T>() {
        Ok(input) => input,
        Err(_) => {
            println!("Invalid input: {}.", input);
            exit(1);
        }
    }
}
