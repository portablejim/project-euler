#![feature(test)]
extern crate test;

use std::fs::File;
use std::io::Read;

fn get_tree(file_name: &str) -> Vec<Vec<u32>> {
    let mut full_file = Vec::new();
    File::open(file_name)
        .expect("Error reading file")
        .read_to_end(&mut full_file)
        .expect("Error during reading file");
    full_file.split(|c| *c as char == '\n')
    // Iterate on lines, splitting numbers
    .map(|line| line.split(|c| *c as char == ' ')
        // Convert 2 chars to a u32
        .map(|n| (n[0] - '0' as u8) as u32 *10 + n[1] as u32 - '0' as u32)
        .collect::<Vec<u32>>())
    .collect()
}

fn longest_length(tree: Vec<Vec<u32>>) -> u32 {
    let mut last_sums: Vec<u32> = Vec::new();

    for v in tree.into_iter() {
        last_sums = match v.len() {
            0 => Vec::new(),
            1 => vec![v[0]],
            2 => vec![v[0] + last_sums[0], v[1] + last_sums[0]],
            _ => {
                let mut temp = vec![v[0] + last_sums[0]];
                for i in 1..(v.len() - 1) {
                    if last_sums[i - 1] >= last_sums[i] {
                        temp.push(last_sums[i - 1] + v[i]);
                    } else {
                        temp.push(last_sums[i] + v[i]);
                    }
                }
                temp.push(last_sums.last().expect("Empty last") + v.last().expect("Empty v"));
                temp
            }
        };
    }

    // Return or error with a shrug.
    *last_sums.iter().max().expect("Max should not be empty ¯\\_(ツ)_/¯")
}

fn main() {
    println!("{}", longest_length(get_tree("task67.txt")));
}

#[bench]
fn bench_file_read(b: &mut test::Bencher) {
    let mut x = Vec::new();
    b.iter(|| x = get_tree("task67.txt"));
}

#[bench]
fn bench_longest_with_file_read(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| x = longest_length(get_tree("task67.txt")));
}
