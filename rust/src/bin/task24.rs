#![feature(test)]
extern crate test;

fn nth_permutation(numbers: Vec<u32>, permute_num: u32) -> Vec<u32> {
    let mut permute = numbers.clone();
    let mut factor: u32 = (1..((numbers.len() + 1) as u32)).product();
    let mut remainder = permute_num - 1;
    for i in 0..numbers.len() {
        factor /= (numbers.len() - i) as u32;
        let next = (remainder / factor) as usize;
        if next != 0 {
            let temp = permute.remove(next + i);
            permute.insert(i, temp);
        }
        remainder -= next as u32 * factor;
    }
    permute
}

fn main() {
    println!("{:?}", nth_permutation((0..10).collect(), 1_000_000));
}

#[bench]
fn bench_collect(b: &mut test::Bencher) {
    let mut x = Vec::new();
    b.iter(|| { x = nth_permutation((0..10).collect(), 1_000_000); });
    println!("{:?}", x);
}
