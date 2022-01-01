fn main() {
    let mut nums: Vec<i32> = vec![1,2,3,4,5];
    let nums_borrowed = &mut nums;
    nums_borrowed[0] = 10;

    println!("{}", nums_borrowed[0]);
}
