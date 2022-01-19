use rand::{thread_rng, Rng}; // import 3rd party crate

// Custom Enums Definitions
enum Result<T, E> {
    OK(T),
    Err(E),
}
use Result::{OK, Err}

enum Error<'a> {
    TooLow{msg: &'a str, val: u32},
    TooHigh{msg: &'a str, val: u32},
}
use Error::{TooHigh, TooLow};

// Function returns a Result enum
fn goldilocks() -> Result<u32, Error<'static>> {
    let mut rng = thread_rng();    
    let value: u32 = rng.gen_range(0..300);
    if value < 100 {return Err(TooLow{msg: "Value is too low", val: value})}
    if value > 200 {return Err(TooHigh{msg: "Value is too High", val: value})}    
    return OK(value);    
}

fn main() {
    let r = goldilocks();
    match r {
        OK(val) => println!("Just Right - {}", val),
        Err(err) => { 
            match err {
                TooLow{msg, val} => println!("{} - {}", msg, val),
                TooHigh{msg, val} => println!("{} - {}", msg, val),
            }
        }
    }    
}
