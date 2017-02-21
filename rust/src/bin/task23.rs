#![feature(test,step_by,collections_bound,btree_range)]
extern crate test;

use std::collections::BTreeSet;
use std::collections::Bound::{Included, Unbounded};

fn brute_factors(input: u32) -> u32 {
    match input {
        0 => 0,
        1 | 2 => 1,
        n => {
            1 +
            (2..(1 + (input as f32).sqrt() as u32))
                .filter(|n| input > 0 && input % n == 0)
                //.filter(|n| input % n == 0)
                //.map(|n| { println!("{} {}", input, n); n })
                .fold(0u32, |s, n| s + n + if n == input / n { 0 } else { input / n })
        }

    }
}

fn brute(target: u32) -> Vec<u32> {
    (1..(target + 1)).map(|n| brute_factors(n)).collect()
}

fn smart(target: u32) -> Vec<u32> {
    let mut sums = Vec::with_capacity(target as usize);
    for i in (0..target) {
        sums.push(1);
    }

    for j in (2..target) {
        for k in ((j * 2)..target).step_by(j) {
            sums[k as usize - 1] += j;
        }
    }

    sums
}

fn non_abundant_sums(limit: u32) -> u32 {
    let abundant_nums = smart(limit)
                 .iter()
                 .enumerate()
                 .filter_map(|(a, b)| if *b > (a + 1) as u32 { Some(1 + a as u32) } else { None })
                 .collect::<Vec<u32>>();
    let mut answer_pool = (0..limit).collect::<BTreeSet<u32>>();
    for outer in (0..abundant_nums.len()) {
        for inner in (outer..abundant_nums.len()) {
            let current = abundant_nums[outer] + abundant_nums[inner];
            if current < limit && answer_pool.contains(&current) {
                answer_pool.remove(&current);
            }
        }
    }
    answer_pool.iter().sum::<u32>()
}

fn main() {
    println!("Ans pool: {:?}", non_abundant_sums(28123));
}

#[bench]
fn bench_brute(b: &mut test::Bencher) {
    let mut x = Vec::new();
    b.iter(|| { x = brute(28123); });
    //println!("{:?}", x);
}

#[bench]
fn bench_smart(b: &mut test::Bencher) {
    let mut x = Vec::new();
    b.iter(|| { x = smart(28123); });
    //println!("{:?}", x);
}

#[bench]
fn bench_non_abundant_sums(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| { x = non_abundant_sums(28123); });
    //println!("{:?}", x);
}
