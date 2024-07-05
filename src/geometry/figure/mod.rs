use std::thread::sleep;
use std::time::Duration;

// This is just for getting a nice effect when drawing
// Set to zero or remove if you don't like it
const SLEEP_TIME: Duration = Duration::from_millis(7);
// Character to mark figure area
const DRAWING_CHAR: char = '#';

/*  Draws a rectangle on the screen.
    The spacing parameter specifies if there will be spacing on the start of the line
    with size height - i (Resulting then in a parallalelogram)
*/
pub fn rectangle(width: u8, height: u8, spacing: bool){

    for i in 0..height{

        if spacing {
            for _ in 0..(height - i) {
                print!(" ");
            }
        }

        for _ in 0..width{
            sleep(SLEEP_TIME);
            print!(" {} ", DRAWING_CHAR);
        }

        print!("\n");
    }
}

/*
  It's a bit boilerplate though, but this draws a 8x8 square with alternating chars
  to "simulate" a chess table and marks a row and column, if specified.
*/
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
            sleep(SLEEP_TIME);
        }

        print!("\n");
    }
}

// This is self-explainable.
pub fn triangle(height: u8){
    for i in 1..height{
        for _ in 0..i{
            print!(" {} ", DRAWING_CHAR);
            sleep(SLEEP_TIME);
        }
        print!("\n");
    }
}