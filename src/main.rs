mod util;
mod business;
mod geometry;
mod math;

use util::{read_line, read_sized, PROMPT};
use business::blockade;
use math::get_quadrant;
use geometry::{draw_figure, chess_game};
use std::process::exit;

fn main() {
    let mut choice = String::new();

    println!("Programs available: ");
    println!("1 - Blockade ");
    println!("2 - Figure drawer");
    println!("3 - Chess marker");
    println!("4 - Get a coordinate's quadrant.");
    print!("{}", PROMPT);

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
        "4" => {
            get_quadrant();
        },
        _ => {
            println!("Invalid choice.");
            exit(1);
        }
    }
}
