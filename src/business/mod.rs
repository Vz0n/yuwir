/*
  Given a street blockade, write a program that computes the price to pay following the next rules:
  - Bikes pays only 10 U.
  - Cars pays 30 U per driven km.
  - Trucks pays 30 U per driven km plus 70 U per transported unit.
  Where U is an arbitrary money unit.
 */

use crate::read_line;
use crate::read_sized;

pub fn blockade(){
    let mut input = String::new();

    println!("Select vehicle type: ");
    
    println!("1 - Bike");
    println!("2 - Car");
    println!("3 - Truck");
    print!("?: ");
    
    match read_line(&mut input){
        "1" => {
            println!("You must pay 10 U.");
        },
        "2" => {
            let kms = read_sized::<u32>(&mut input, "Enter driven kilometers: ");

            let total = kms*30;
            println!("You must pay {} U.", total);
        },
        "3" => {
            let kms = read_sized::<u32>(&mut input, "Enter driven kilometers: ");
            let tons = read_sized::<u32>(&mut input, "Enter transported tons: ");

            let total = kms*30 + tons*70;
            println!("You must pay {} U.", total);
        }
        _ => {
            println!("Invalid choice.");
        }
    }
}