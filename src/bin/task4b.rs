
trait Reversable {
    fn reverse(&self) -> u64;
}

impl Reversable for u64 {
    fn reverse(&self) -> u64 {
        reverse_int(*self, 0)
    }
}

fn reverse_int(inp: u64, out: u64) -> u64 {
    match (inp / 10, inp % 10) {
        (0, r) => out * 10 + r,
        (d, r) => reverse_int(d, out * 10 + r)
    }
}

fn max_palindrome_product(digits: u32) -> u64 {
    let max = (10 as u64).pow(digits) - 1;
    let diff = max - (max/10);
    (0..diff).map(|n| max - n )
    .filter_map(|n| {
        (0..((max - n) / 2 + 1))
        .map(|off| (max - off) * (n+off))
        .filter(|c| *c == c.reverse())
        .max()
    })
    // Get first only
    .take(1).last()
    // If palindrome not in search space, return 0
    .unwrap_or(0)
}

fn main() {
    let max = 8;
    println!("{:?}", max_palindrome_product(max));
}