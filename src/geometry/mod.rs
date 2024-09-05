mod figure;


use crate::{read_line, read_sized, PROMPT};
use ansi_term::Style;
use figure::{rectangle,triangle, chess_table};

/*
  [EXTRA]: Now added a parallelogram
  Write a program that can draw a rectangle triangle, square and rectangle with sizes selected by the user.
  Example of usage:
  Select figure: 1
  Enter square size: 4

  # # # # #
  # # # # #
  # # # # #
  # # # # #

  ... END
*/
pub fn draw_figure(style: Style){

    let mut input = String::new();

    println!("{}", style.paint("Select figure to draw"));
    println!("1 - Square");
    println!("2 - Rectangle");
    println!("3 - Rectangle triangle");
    println!("4 - Parallelogram");
    print!("{}", style.paint(PROMPT));

    match read_line(&mut input){
        "1" => {
            let size = read_sized::<u8>(&mut input, "Enter size:");

            print!("\n");
            rectangle(size, size, false);
        },
        "2" => {
            let width = read_sized::<u8>(&mut input, "Enter width: ");
            let height = read_sized::<u8>(&mut input, "Enter height: ");

            print!("\n");
            rectangle(width, height, false);
        }
        "3" => {
            let size = read_sized::<u8>(&mut input, "Enter size: ");

            print!("\n");
            triangle(size);
        }
        "4" => {
            let width = read_sized::<u8>(&mut input, "Enter width: ");
            let height = read_sized::<u8>(&mut input, "Enter height: ");

            print!("\n");
            rectangle(width, height, true);
        }
        _ => {
            println!("Invalid choice.");
        }
    }
}

/*
  Write a program that draws an 8x8 chess table marking every white and black position with a respective char,
  and let the user mark a point in the table.
  Example of usage:
  W B W B W B W B
  ... [snip]
  Enter row: 1
  Enter column: 2
  W * W B W B W B
  ... [snip]
 */
pub fn chess_game(){
    let mut input = String::new();
    let white = 'W';
    let black = 'B';

    // Just put a number out of bounds to print the entire table
    // without the marker
    chess_table(white, black, 255, 255);
    
    let row = read_sized::<u8>(&mut input, "Enter row: ");
    let column  = read_sized::<u8>(&mut input, "Enter column: ");

    chess_table(white, black, row, column)
}