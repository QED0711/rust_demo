use std::f64::consts::PI; // import from standard libary

// Define enum (each can have individual properties)
enum Shape {
     Rectangle { height: u32, width: u32 },
     Circle { radius: u32 },
}

use Shape::{Rectangle, Circle}; // extract from enum namespace

fn main() { 
    let rect = Shape::Rectangle {height: 5, width: 10};
    let circ = Shape::Circle {radius: 3};

    print_area(&rect);
    print_area(&circ);
    
}

fn print_area(shape: &Shape) { // pattern matching utility function
    match shape {
        Rectangle {height, width} => println!("{}", height*width),
        Circle {radius} => println!("{}", PI * (*radius as f64).powf(2 as f64))
    }
}