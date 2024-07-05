mod util;
mod business;
mod geometry;

use util::{read_line, read_sized};
use business::blockade;
use geometry::{draw_figure, chess_game};
use std::process::exit;

fn main() {
    let mut choice = String::new();

    println!("Programs available: ");
    println!("1 - Blockade ");
    println!("2 - Figure drawer");
    println!("3 - Chess marker");
    print!("?: ");

    match read_line(&mut choice){
        "1" => {
            blockade();
        },
        "2" => {
            draw_figure();
        },
        "3" => {
            chess_game();
        },
        _ => {
            println!("Invalid choice.");
            exit(1);
        }
    }
}
