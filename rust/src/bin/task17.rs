#![feature(test)]
extern crate test;

fn main() {
    let to_1_000 = (1..1000).map(|i| word(i).len()).sum::<usize>();
    let len_1_000 = "onethousand".len();
    println!("{}", to_1_000 + len_1_000);
}

fn word<'a>(n: u32) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        20...29 => format!("twenty{}", word(n%10)),
        30...39 => format!("thirty{}", word(n%10)),
        40...49 => format!("forty{}", word(n%10)),
        50...59 => format!("fifty{}", word(n%10)),
        80...89 => format!("eighty{}", word(n%10)),
        60...79 | 90...99 => format!("{}ty{}", word(n/10), word(n%10)),
        x if x >= 100 && x % 100 == 0 => format!("{}hundred", word(n/100)),
        x if x >= 100 && x % 100 != 0 => format!("{}hundredand{}", word(n/100), word(n%100)),
        _ => "".to_string()
    }
}

#[bench]
fn bench_string(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| { 
        let to_1_000 = (1..1000).map(|i| word(i).len()).sum::<usize>();
        let len_1_000 = "onethousand".len();
        x = to_1_000 + len_1_000;
    });
    println!("{}", x);
}
