#![feature(test,step_by)]
extern crate test;
extern crate num;

use num::integer::lcm;

fn hand_gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn hand_lcm(a: u32, b: u32) -> u32 {
    a / hand_gcd(a,b) * b
}

fn main() {
    println!("{:?}", (1..21).fold(1, |a,b| hand_lcm(a,b)));
}

#[bench]
fn bench_brute_force(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(move || {
        x = (2520..).step_by(20).filter(|c| {
            (1..21).all(|n| c % n == 0)
        }).next().expect("No value");
    });
    println!("{:?}", x);
}

#[bench]
fn bench_lcm(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(move || {
        x = (1..21).fold(1, |a,b| num::integer::lcm(a,b));
    });
    println!("{:?}", x);
}

#[bench]
fn bench_hand_lcm(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(move || {
        x = (1..21).fold(1, |a,b| hand_lcm(a,b));
    });
    println!("{:?}", x);
}