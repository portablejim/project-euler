#![feature(test)]
extern crate test;

use std::fs::File;
use std::io::Read;

fn get_names(file_name: &str) -> Vec<String> {
    let full_names = {
        let mut temp = String::new();
        File::open(file_name)
            .expect("File open failed")
            .read_to_string(&mut temp)
            .expect("Reading file failed");
        temp
    };
    full_names.split(|c| c as char == ',')
    .map(|w| &w[1..(w.len()-1)])
    .map(|w| w.to_string())
    .collect()

}

fn get_sorted_names(names: Vec<String>) -> Vec<String> {
    let mut output = names;
    output.sort();
    output
}

fn get_names_scores(names: Vec<String>) -> u32 {
    names.into_iter()
        .enumerate()
        .map(|(num, name)| {
            name.bytes().map(|c| ((c - ('A' as u8) + 1) as u32)).sum::<u32>() * (num + 1) as u32
        })
        .sum()
}

fn main() {
    println!("{:?}",
             get_names_scores(get_sorted_names(get_names("task22.txt"))));
}

#[bench]
fn bench_1(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| x = get_names_scores(get_sorted_names(get_names("task22.txt"))));
}
