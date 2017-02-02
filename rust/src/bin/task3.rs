#![feature(i128_type)]

fn get_candidates(num: i128) -> Vec<i128> {
    let max = (num as f64).sqrt() as i128 + 1;
    (2..(max+1)).filter(|x| *x == 2 || *x % 2 == 1).collect()
}

fn is_prime(num: i128) -> bool {
    get_candidates(num).into_iter()
                       .all(|n| num % n != 0)

}

fn pollard(num: i128) -> Option<i128>{
    fn g(x: i128, n: i128) -> i128 { (x*x + 1) % n }
    fn gcd(x: i128, y: i128) -> i128 {
        let (a, b) = match x >= y {
            true => (x, y),
            false => (y, x)
        };
        match b {
            0 => a,
            _ => gcd(b, a % b)
        }
    }

    let mut x = 2;
    let mut y = 2;
    let mut d = 1;
    while d == 1 {
        x = g(x, num);
        y = g(g(y, num), num);
        d = gcd((x - y).abs(), num);
    }
    match d == num {
        true => None,
        false => Some(d)
    }

}

fn prime(num: i128) -> i128 {
    let p_o = pollard(num);
    match p_o {
        Some(p) => {
            match is_prime(num/p) {
                true => num / p,
                false => {
                    prime(num / p)
                }
            }
        },
        None => num
    }
}

fn main() {
    println!("{:?}", prime(600851475143));
}