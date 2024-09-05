mod util;
mod business;
mod geometry;
mod math;

use util::{read_line, read_sized, PROMPT};
use ansi_term::Color::Purple;
use business::blockade;
use math::get_quadrant;
use geometry::{draw_figure, chess_game};
use std::process::exit;

fn main() {
    let mut choice = String::new();
    let style = Purple.normal();

    println!("{} ", style.paint("Programs available:"));
    println!("1 - Blockade ");
    println!("2 - Figure drawer");
    println!("3 - Chess marker");
    println!("4 - Get a coordinate's quadrant.");
    print!("{}", style.paint(PROMPT));

    match read_line(&mut choice){
        "1" => {
            blockade(style);
        },
        "2" => {
            draw_figure(style);
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
