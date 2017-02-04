fn main() {
    let sum_of_squares: i32 = (1..101).map(|a| a*a ).sum();
    let square_of_sum = 101*50*101*50;
    println!("{:?}", square_of_sum - sum_of_squares);
}