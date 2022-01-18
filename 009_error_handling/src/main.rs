use rand::Rng; // import 3rd party crate

// Implement our own result enum
enum Result<T, E> {
    OK(T),
    Err(E),
}

// Returns a number if it is greater than 100, else an error
fn gen_above_100() -> Result<u8, &'static str> {
    let mut rng = rand::thread_rng();
    let score: u8 = rng.gen();

    if score > 100 {
        return Result::OK(score);
    } else {
        return Result::Err("score not high enough");
    }
}

fn main() {
    let r = gen_above_100();
    match r {
        Result::OK(val) => println!("SUCCESS: {}", val),
        Result::Err(err_msg) => println!("ERROR: {}", err_msg)
    }    
}
