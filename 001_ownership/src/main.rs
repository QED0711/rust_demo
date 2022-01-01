fn main() {
    let nums: Vec<i32> = vec![1,2,3,4,5];
    let nums_borrowed = &nums;
    println!("{}", nums_borrowed[0]);
}
