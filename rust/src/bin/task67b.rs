#![feature(test)]
extern crate test;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread;

trait From0 {
    fn from0(self) -> u32;
}

impl From0 for u8 {
    fn from0(self) -> u32 {
        if self >= ('0' as u8) {
            self as u32 - '0' as u32
        } else {
            0
        }
    }
}

fn get_tree(file_name: &str, tx: Sender<Vec<u32>>) {
    let f = File::open(file_name).expect("Cannot open file");
    let file = BufReader::new(&f);
    for l in file.lines() {
        tx.send(l.unwrap()
                .split(|c| c == ' ')
                .map(|s| s.trim().parse::<u32>().unwrap_or(0))
                .collect::<Vec<u32>>())
            .expect("Send didn't work");
    }
}

fn longest_length(rx: Receiver<Vec<u32>>) -> u32 {
    let mut last_sums: Vec<u32> = Vec::new();

    while let Ok(v) = rx.recv() {
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

fn get_longest_length(filename: &'static str) -> u32 {
    let (tx, rx) = channel();

    thread::spawn(move || get_tree(filename, tx));
    let child2 = thread::spawn(|| longest_length(rx));
    child2.join().expect("Child 2 did not return a result")
}

fn main() {
    println!("{}", get_longest_length("task67.txt"));
}

#[bench]
fn bench_longest_with_file_read(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| x = get_longest_length("task67.txt"));
}
