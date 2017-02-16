extern crate num;

use num::Integer;
use num::bigint::BigInt;
use num::integer::gcd;
use num::integer::lcm;

fn main() {
    let number1 = 500;
    let lcm_num1 = (2..(number1+1)).fold(BigInt::from(1), |a,b| { println!("{:?}", a); a.lcm(&BigInt::from(b))});
    println!("{}", lcm_num1);
    println!("{}", lcm(2,3));
}