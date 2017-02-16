#![feature(test)]
extern crate test;
extern crate num;

use num::Integer;
use num::bigint::BigInt;
use num::ToPrimitive;
use std::borrow::Borrow;

fn calc_with_bignum() -> u32 {
    let mut fac = (2..100).fold(BigInt::from(1), |a, b| a * BigInt::from(b));
    let ten = BigInt::from(10);
    let one = BigInt::from(1);
    let mut sum = 0;
    while fac > *one.borrow() {
        let mut div = &one;
        let mut rem = 0;
        let (div, rem) = fac.div_rem(&ten);
        fac = div;
        sum += rem.to_u32().expect("10 is too big");
    }
    sum
}

fn main() {
    println!("{:?}", calc_with_bignum());
}

#[bench]
fn bench_bignum(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| { x = calc_with_bignum(); });
    println!("{:?}", x);
}