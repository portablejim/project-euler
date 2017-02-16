#![feature(i128_type,test)]
extern crate test;

fn factorial_iterative(n: u128, to: u128) -> u128 {
    ((to+1)..n+1).fold(1, |p, n| p*n)
}

fn choose(n: u128, k: u128) -> u128 {
    factorial_iterative(n, k) / factorial_iterative(k, 0)
}

fn matrix(x: u128, y: u128) -> u128 {
    choose(x+y, y)
}

fn main() {
    println!("{}", matrix(20,20));
}

#[bench]
fn bench_grid20(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| { x = matrix(20,20); x });
    println!("{}", x);
}