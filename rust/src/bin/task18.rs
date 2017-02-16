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
    println!("{}", longest_length(get_tree("task18.txt")));
}

#[bench]
fn bench_file_read(b: &mut test::Bencher) {
    let mut x = Vec::new();
    b.iter(|| x = get_tree("task18.txt"));
}

#[bench]
fn bench_longest_with_file_read(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| x = longest_length(get_tree("task18.txt")));
}

#[bench]
fn bench_longest_with_provided(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(move || {
        let input = vec![vec![75],
                         vec![95, 64],
                         vec![17, 47, 82],
                         vec![18, 35, 87, 10],
                         vec![20, 4, 82, 47, 65],
                         vec![19, 1, 23, 75, 3, 34],
                         vec![88, 2, 77, 73, 7, 63, 67],
                         vec![99, 65, 4, 28, 6, 16, 70, 92],
                         vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
                         vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
                         vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
                         vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
                         vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
                         vec![63, 66, 4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
                         vec![4, 62, 98, 27, 23, 9, 70, 98, 73, 93, 38, 53, 60, 4, 23]];
        x = longest_length(input)
    });
}
