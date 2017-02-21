#![feature(test)]
extern crate test;

fn amicable(input: u32) -> u32 {
    1 +
    (2..((input as f32).sqrt() as u32))
        .filter(|n| input > 0 && input % n == 0)
        .fold(0u32, |s, n| s + n + input / n)
}

fn main() {
    let numbers: Vec<u32> =
        (1..10_000).filter(|n| amicable(amicable(*n)) == *n && amicable(*n) != *n).collect();
    println!("Amicable numbers: {:?}", numbers);
    println!("Sum: {}", numbers.iter().sum::<u32>());
}

#[bench]
fn bench_collect(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| {
        x = (1..10_000)
            .filter(|n| amicable(amicable(*n)) == *n && amicable(*n) != *n)
            .collect::<Vec<u32>>()
            .iter()
            .sum();
    });
    println!("{:?}", x);
}

#[bench]
fn bench_sum(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| {
        x = (1..10_000)
            .filter(|n| { let t = amicable(*n); t != *n && amicable(t) == *n })
            .sum();
    });
    println!("{:?}", x);
}
