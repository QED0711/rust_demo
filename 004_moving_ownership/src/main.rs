fn main() {
    let my_vec = vec![1,2,3,4,5];
    let multiplier = 10;

    let multiplied = multiply_vec(my_vec, multiplier); // owned my_vec passed as argument, ownership transfered to new scope, then to multiplied

    println!("{:?}", multiplied);
}

fn multiply_vec(input_vec: Vec<i32>, multiplier: i32) -> Vec<i32> {
    let mut working_vec = input_vec; // input_vec is owner, transfers ownership to mutable working_vec
    let rng = 0..working_vec.len(); 
    for i in rng{
        working_vec[i] = working_vec[i] * multiplier; // directly mutate owned data
    }
    working_vec // return owned data. Ownership transfers to outer scope
}
