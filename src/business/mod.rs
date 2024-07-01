/*
  Given a street blockade, write a program that computes the price to pay following the next rules:
  - Bikes pays only 10 U.
  - Cars pays 30 U per driven km.
  - Trucks pays 30 U per driven km plus 70 U per transported unit.
  Where U is an arbitrary money unit.
 */

use crate::read_line;
use crate::exit;

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
            print!("Enter driven kilometers: ");
            let kms: i32 = match read_line(&mut input).parse::<i32>() {
                Ok(num) => {
                    num
                },
                Err(_) => {
                    println!("Invalid number entered.");
                    exit(1);
                },
            };

            let total = kms*30;
            println!("You must pay {} U.", total);
        },
        "3" => {
            print!("Enter driven kilometers: ");
            let kms: i32 = match read_line(&mut input).parse::<i32>() {
                Ok(num) => {
                    num
                },
                Err(_) => {
                    println!("Invalid number entered.");
                    exit(1);
                },
            };

            print!("Enter transported tons: ");
            let tons: i32 = match read_line(&mut input).parse::<i32>() {
                Ok(num) => {
                    num
                },
                Err(_) => {
                    println!("Invalid number entered.");
                    exit(1);
                },
            };

            let total = kms*30 + tons*70;
            println!("You must pay {} U.", total);
        }
        _ => {
            println!("Invalid choice.");
        }
    }
}