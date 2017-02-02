
trait Reversable {
    fn reverse(&self) -> i32;
}

impl Reversable for i32 {
    fn reverse(&self) -> i32 {
        reverse_int(*self, 0)
    }
}

fn reverse_int(inp: i32, out: i32) -> i32 {
    match (inp / 10, inp % 10) {
        (0, r) => out * 10 + r,
        (d, r) => reverse_int(d, out * 10 + r)
    }
}

fn max_palindrome_product(max: i32) -> i32 {
    (0..(max - (max/10+1))).map(|n| max - n)
            .flat_map(|i| {
                (i..max+1).map(|n| i + (max - n))
                .map(|m| i * m)
                .filter(|c| *c == c.reverse())
                .collect::<Vec<i32>>()
            }).max()
            .unwrap_or(0) //.collect::<Vec<i32>>();
}

fn main() {
    let max = 999;
    let a = max_palindrome_product(max);
    println!("{:?}", a);
}