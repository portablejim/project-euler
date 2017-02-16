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
    let mut tree_sums: Vec<Vec<u32>> = Vec::new();
    tree_sums.push(tree[tree.len() - 1].clone());
    for row in (0..(tree.len() - 1)).rev() {
        let mut temp: Vec<u32> = Vec::new();
        for i in 0..(row + 1) {
            let prev = tree_sums.last().expect("Tree sums is empty");
            if prev[i] > prev[i + 1] {
                temp.push(tree[row][i] + prev[i])
            } else {
                temp.push(tree[row][i] + prev[i + 1])
            }
        }
        tree_sums.push(temp);
    }
    // Return or error with a shrug.
    tree_sums.last().expect("Tree sums should not be empty ¯\\_(ツ)_/¯")[0]
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
