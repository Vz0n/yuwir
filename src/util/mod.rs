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

pub fn read_sized<T: FromStr>(output: &mut String, dialog: &str) -> T{
    print!("{}", dialog);

    match read_line(output).parse::<T>() {
        Ok(output) => output,
        Err(_) => {
            println!("Invalid input: {}.", output);
            exit(1);
        }
    }
}
