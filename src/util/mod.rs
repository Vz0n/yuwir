use std::io::{stdin, stdout, Write};

pub fn read_line(buff: &mut String) -> &str{
    // First, flush stdout and clear buffer
    buff.clear();
    let _ = stdout().flush();

    // Now, read and write
    let _ = stdin().read_line(buff);
    buff.trim()
}
