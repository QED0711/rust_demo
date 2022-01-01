fn main() {
    let val1: Option<i32> = None;
    let val2: Option<i32> = Some(1);

    println!("val1 is now {}", increment(&val1));
    println!("val2 is now {}", increment(&val2));
}

fn increment(v: &Option<i32>) -> i32 {
    match v {
        None => 1,
        Some(i) => i + 1
    }
}
