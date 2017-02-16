#![feature(test)]
extern crate test;
use std::sync::mpsc::{channel, Sender, Receiver};

#[inline]
fn collatz(i: i64) -> i64 {
    (i % 2) * (3 * i + 1) + (1 - (i % 2)) * (i / 2)
}

fn collatz_recursive(mut vals: &mut [i64], num: i64) -> i64 {
    match collatz(num) {
        1 => 1,
        n if num < vals.len() as i64 && n < vals.len() as i64 && vals[n as usize] == 0 => {
            vals[num as usize] = 1 + collatz_recursive(&mut vals, n);
            vals[num as usize]
        }
        n if n < vals.len() as i64 && vals[n as usize] > 0 => 1 + vals[n as usize],
        n if n >= vals.len() as i64 || num >= vals.len() as i64 => {
            1 + collatz_recursive(&mut vals, n)
        }
        _ => panic!("Collatz Iter error"),
    }
}


fn do_collatz() -> (i64, i64) {
    let mut values = vec![0;1_000_000];
    let mut max = (1, 1);

    // Init a few values
    values[1] = 1;
    values[2] = 2;

    for n in 3..1_000_000 {
        let mut count = 1;
        let mut cur = n;
        match collatz_recursive(&mut values, n) {
            c if c > max.1 => max = (n, c),
            _ => (),
        }
    }
    max
}


fn main() {
    println!("{:?}", do_collatz());
}

#[bench]
fn bench_recursive(b: &mut test::Bencher) {
    b.iter(|| do_collatz());
}
