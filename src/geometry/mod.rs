mod figure;


use crate::read_line;
use crate::read_sized;
use figure::{rectangle,triangle, chess_table};

/*
  Write a program that can draw a rectangle triangle, square and rectangle with sizes selected by the user.
  Example of usage:
  Select figure: 1
  Enter square size: 4
  [EXTRA]: Now added a parallelogram

  # # # # #
  # # # # #
  # # # # #
  # # # # #

  ... END
*/
pub fn draw_figure(){

    let mut input = String::new();

    println!("Select figure to draw");
    println!("1 - Square");
    println!("2 - Rectangle");
    println!("3 - Rectangle triangle");
    println!("4 - Parallelogram");
    print!("?: ");

    match read_line(&mut input){
        "1" => {
            print!("Enter size: ");
            let size = read_sized::<u8>(&mut input);

            print!("\n");
            rectangle(size, size, false);
        },
        "2" => {
            print!("Enter width: ");
            let width = read_sized::<u8>(&mut input);

            print!("Enter height: ");
            let height = read_sized::<u8>(&mut input);

            print!("\n");
            rectangle(width, height, false);
        }
        "3" => {
            print!("Enter size: ");
            let size = read_sized::<u8>(&mut input);

            print!("\n");
            triangle(size);
        }
        "4" => {
            print!("Enter width: ");
            let width = read_sized::<u8>(&mut input);

            print!("Enter height: ");
            let height = read_sized::<u8>(&mut input);

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
    
    print!("Enter row: ");
    let row = read_sized::<u8>(&mut input);

    print!("Enter column: ");
    let column  = read_sized::<u8>(&mut input);

    chess_table(white, black, row, column)
}