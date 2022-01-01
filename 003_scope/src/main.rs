fn main() {
    let vec_1 = vec![1,2,3,4,5]; // vec_1 alocated to memory
    let multiplier = 10;

    {
        let mut vec_2 = Vec::new(); // vec_2 alocated to memory
        for i in &vec_1{ // vec_1 borrowed
            vec_2.push(i * multiplier); // vec_2 mutated
        }
        println!("{:?}", vec_2); // vec_2 still valid here
    } // inner scope closes, vec_2 dealocated 

    println!("{:?}", vec_1); // vec_1 still valid here
}
