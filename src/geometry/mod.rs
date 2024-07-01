mod figure;

use crate::read_line;
use crate::exit;
use figure::{rectangle,triangle, chess_table};

fn read_sized_number(input: &mut String) -> u8{
    match read_line(input).parse::<u8>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid size entered: {}. please only use numbers between 1 and 255.", input);
            exit(1);
        }
    }
}

/*
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
pub fn draw_figure(){

    let mut input = String::new();

    println!("Select figure to draw");
    println!("1 - Square");
    println!("2 - Rectangle");
    println!("3 - Rectangle triangle.");
    print!("?: ");

    match read_line(&mut input){
        "1" => {
            print!("Enter size: ");
            let size: u8 = read_sized_number(&mut input);

            rectangle(size, size);
        },
        "2" => {
            print!("Enter width: ");
            let width: u8 = read_sized_number(&mut input);

            print!("Enter height: ");
            let height: u8 = read_sized_number(&mut input);

            rectangle(width, height);
        }
        "3" => {
            print!("Enter size: ");
            let size: u8 = read_sized_number(&mut input);

            triangle(size);
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
    let row: u8 = read_sized_number(&mut input);

    print!("Enter column: ");
    let column: u8 = read_sized_number(&mut input);

    chess_table(white, black, row, column)
}