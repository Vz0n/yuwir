// Limit the size to u8 because of the terminal's screen.
pub fn rectangle(width: u8, height: u8){
    for _ in 0..height{
        for _ in 0..width{
            print!(" # ");
        }

        print!("\n");
    }
}

pub fn chess_table(white: char, black: char, row: u8, column: u8){
    for i in 0..8{
        for j in 0..8{
            // Add a difference of one to print the marker exactly in the
            // specified position.
            if (row - 1) == i && (column - 1) == j {
                print!(" * ");
                continue;
            } 

            // Alternate between even and odd row/columns of the "matrix"
            if (i + j) % 2 == 0 {
                print!(" {} ", white);
            } else {
                print!(" {} ", black);
            }
        }

        print!("\n");
    }
}

pub fn triangle(height: u8){
    for i in 1..height{
        for _ in 0..i{
            print!(" # ");
        }
        print!("\n");
    }
}