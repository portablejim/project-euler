#![feature(advanced_slice_patterns)]

fn fib(arr: &mut Vec<i32>) -> Vec<i32> {
    match arr.len() {
        l if l >= 2 => match arr[l-2] + arr[l-1] {
            s if s < 4_000_000 => { arr.push(s); fib(arr) },
            _ => arr.clone()
        },
        _ => Vec::<i32>::new()
    }
}

fn main() {
    let mut init = vec![1,2];
    let f = fib(&mut init);
    println!("{:?}", f.iter().filter(|i| *i % 2 == 0).sum::<i32>())
}