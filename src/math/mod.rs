/*
  Write a program that can identify the cuadrant in which a point lies in the 2D plane (R^2).
*/

use crate::read_sized;

pub fn get_quadrant(){
    let mut input = String::new();
    let quadrant: u8;

    let x = read_sized::<u8>(&mut input, "Enter value for x: ");
    let y = read_sized::<u8>(&mut input, "Enter value for y: ");

    if x > 0 {
        if y > 0 {
            quadrant = 1;
        } else {
            quadrant = 4;
        }
    } else {
        if y > 0{
            quadrant = 2;
        } else {
            quadrant = 3;
        }
    }

    println!("The coordinate is on the quadrant {}.", quadrant);
}
