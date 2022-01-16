fn main() {
    let num_1 = 5;
    let num_2 = 10;
    println!("{}", largest(&num_1, &num_2));
}

// 'a is a lifetime indicator. It tells the compiler that this function expects two variables that will be valid for some period 'a
fn largest<'a>(num_1: &'a i32, num_2: &'a i32) -> &'a i32 { 
    if num_1 > num_2 {
        &num_1
    } else {
        &num_2
    }
}
