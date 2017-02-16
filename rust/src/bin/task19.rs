#![feature(test)]
extern crate test;

use std::thread;
use std::sync::mpsc::channel;

#[derive(Clone,Debug)]
struct Year {
    next: usize,
    first_days: Vec<usize>,
}
struct Year2 {
    next: usize,
    first_days: [usize; 12],
}

impl Year {
    fn get_months(&self, offset: usize) -> Vec<usize> {
        self.first_days.iter().map(|m| (m + offset) % 7).collect()
    }
}

impl Year2 {
    fn get_months(&self, offset: usize) -> Vec<usize> {
        self.first_days.iter().map(|m| (m + offset) % 7).collect()
    }
}

fn get_sundays_iterative_cache() -> u32 {
    let non_leap_year = Year {
        next: 1,
        first_days: vec![0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5],
    };
    let leap_year = Year {
        next: 2,
        first_days: vec![0, 3, 4, 0, 2, 5, 7, 3, 6, 1, 4, 6],
    };

    let mut cache: [i16; 7] = [-1, -1, -1, -1, -1, -1, -1];

    let mut count = 0;
    let mut offset = 2;
    for year_num in 1901..2001 {
        let year = if year_num % 4 == 0 && (year_num % 100 != 0 || year_num % 400 == 0) {
            &leap_year
        } else {
            &non_leap_year
        };

        if cache[offset] < 0 {
            cache[offset] = year.get_months(offset).iter().filter(|o| **o == 0).count() as i16;
        }
        count += cache[offset] as u8;

        offset = (offset + year.next) % 7;
    }
    count as u32
}

static NON_LEAP_YEAR: Year2 =  Year2 {
    next: 1,
    first_days: [0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5],
};
static LEAP_YEAR: Year2  = Year2 {
    next: 2,
    first_days: [0, 3, 4, 0, 2, 5, 7, 3, 6, 1, 4, 6],
};

fn get_sundays_parallel_cache<'a>() -> u32 {
    let non_leap_year = Year {
        next: 1,
        first_days: vec![0, 3, 3, 6, 1, 4, 6, 2, 5, 0, 3, 5],
    };
    let leap_year = Year {
        next: 2,
        first_days: vec![0, 3, 4, 0, 2, 5, 7, 3, 6, 1, 4, 6],
    };

    let (tx, rx) = channel();

    thread::spawn(move || {
        let mut offset = 2;
        for year_num in 1901..2001 {
            let year = if year_num % 4 == 0 && (year_num % 100 != 0 || year_num % 400 == 0) {
                &LEAP_YEAR
            } else {
                &NON_LEAP_YEAR
            };

            tx.send((year, offset));

            offset = (offset + year.next) % 7;
        }
    });

    let mut cache: [i16; 7] = [-1, -1, -1, -1, -1, -1, -1];
    let mut count = 0;

    while let Ok((year, offset)) = rx.recv() {
        if cache[offset] < 0 {
            cache[offset] = (year as &Year2).get_months(offset).iter().filter(|o| **o == 0).count() as i16;
        }
        count += cache[offset] as u8;
    }

    count as u32
}

fn main() {
    println!("{}", get_sundays_iterative_cache());
    println!("{}", get_sundays_parallel_cache());
}

#[bench]
fn bench_iterative_cache(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| { x = get_sundays_iterative_cache(); });
    println!("{:?}", x);
}

#[bench]
fn bench_parallel(b: &mut test::Bencher) {
    let mut x = 0;
    b.iter(|| { x = get_sundays_parallel_cache(); });
    println!("{:?}", x);
}
