#![feature(test)]
extern crate test;

use std::fs::File;
use std::io::Read;
use std::io::BufReader;
use std::io::BufRead;

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

fn get_tree_b(file_name: &str) -> Vec<Vec<u32>> {
    let f = File::open(file_name).expect("Cannot open file");
    let file = BufReader::new(&f);
    file.lines().map(|l| {
        l.unwrap().split(|c| c == ' ')
                .map(|s| s.trim().parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>()
            //.expect("Send didn't work");
    }).collect()
    //out
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

fn longest_length_b(tree: Vec<Vec<u32>>) -> u32 {
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

    // Return or error with a shrug.*/
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
fn bench_file_read_b(b: &mut test::Bencher) {
    let mut x = Vec::new();
    b.iter(|| x = get_tree_b("task67.txt"));
}

#[bench]
fn bench_longest_with_file_read(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| x = longest_length(get_tree("task67.txt")));
}

#[bench]
fn bench_longest_with_file_read_b(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| x = longest_length_b(get_tree("task67.txt")));
}
