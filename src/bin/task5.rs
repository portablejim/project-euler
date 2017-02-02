fn main() {
    let fil = (2520..).filter(|c| {
        (1..21).all(|n| c % n == 0)
    }).next();
    println!("{:?}", fil);
}