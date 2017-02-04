#![feature(test,step_by)]

extern crate test;

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::ops::DerefMut;

// From functional infinite sieve paper
// https://www.cs.hmc.edu/~oneill/papers/Sieve-JFP.pdf
const RING2357: &'static [i32; 48] = &[2,4,2,4,6,2,6,4,2,4,6,6,2,6,4,2,6,4,6,8,4,2,4,2,4,8,6,4,6,2,4,6,2,6,6,4,2,4,6,2,6,4,2,4,2,10,2,10];

struct Ring2357 {
    offset: usize,
}

trait Ring {
    fn should_skip(target: i32) -> bool;
    fn next(&mut self) -> Option<i32> ;
}

impl Ring2357 {
    fn new() -> Ring2357 {
        Ring2357 { offset: RING2357.len()-1 }
    }
}

impl Ring for Ring2357 {
    fn next(&mut self) -> Option<i32> {
        self.offset = (self.offset + 1) % RING2357.len();
        //println!("Ring - {}", RING2357[self.offset]);
        Some(RING2357[self.offset])
    }

    fn should_skip(target: i32) -> bool {
        target % 2 == 0
        || target % 3 == 0
        || target % 5 == 0
        || target % 7 == 0
    }
}

struct SteppingCounter {
    current: i32,
    maximum: i32,
    it: Ring2357,
}

impl SteppingCounter {
    fn new_from(start: i32, iterator: Ring2357) -> SteppingCounter {
        SteppingCounter { current: start, maximum: std::i32::MAX, it: iterator }
    }

    /*fn new_from_to(start: i32, end: i32, iterator: Ring2357) -> SteppingCounter {
        SteppingCounter { current: start, maximum: end, it: iterator }
    }*/
}

impl Iterator for SteppingCounter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        match self.it.next() {
            None => None,
            Some(x) if x > self.maximum => None,
            Some(x) => match x { 
                n if n > self.maximum => None,
                n => { self.current += n; Some(self.current - n) },
            }
        }
    }
}

#[derive(Eq,Copy)]
struct WeightedRange(i32, i32);

impl WeightedRange {
    fn new(start: i32, jump: i32) -> WeightedRange {
        WeightedRange(start, jump)
    }

    fn next(&self) -> Self {
        WeightedRange(self.0 + self.1, self.1)
    }

    fn next_mut(&mut self) {
        while {
            self.0 += self.1;
            false //Ring2357::should_skip(self.0)
        } { /* DO WHILE */ }
    }
}

impl Clone for WeightedRange {
    fn clone(&self) -> WeightedRange { *self }
}

impl Ord for WeightedRange {
    fn cmp(&self, other: &WeightedRange) -> Ordering {
        match self.0.cmp(&other.0) {
            // Need least values, but PriorityQueue/BinaryHeap only provides
            // access to most values. So opposites day begins.
            Ordering::Less => Ordering::Greater,
            Ordering::Equal => Ordering::Equal,
            Ordering::Greater => Ordering::Less
        }
    }
}
impl PartialOrd for WeightedRange {
    fn partial_cmp(&self, other: &WeightedRange) -> Option<Ordering> {
        match self.0.cmp(&other.0) {
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => Some(Ordering::Equal),
            Ordering::Greater => Some(Ordering::Less)
        }
    }
}
impl PartialEq for WeightedRange {
    fn eq(&self, other: &WeightedRange) -> bool {
        self.0 == other.0
    }
}

fn filter_prime(cp: i32, composite: &mut BinaryHeap<WeightedRange>) -> bool {
    match composite.peek().cloned() {
        // Nothing here, so add the square.
        None => { let mut new_wr = WeightedRange::new(cp, cp); new_wr.next_mut(); &mut composite.push(new_wr); true },
        // Number is composite, so go to the next one.
        Some(wr) if wr.0 == cp => { 
            while composite.peek().and_then(|cwr| Some(cwr.0)) == Some(wr.0) {
                match composite.peek_mut() {
                    None => (),
                    Some(mut temp_wr) => { temp_wr.deref_mut().next_mut() }
                };
            };
            false
        },
        Some(wr) if wr.0 < cp => { 
            while composite.peek().and_then(|cwr| Some(cwr.0)) < Some(cp) {
                //println!("Less: {} + {}", composite.peek().unwrap().0, composite.peek().unwrap().1);
                match composite.peek_mut() {
                    None => (),
                    Some(mut temp_wr) => { temp_wr.deref_mut().next_mut() }
                };
            };
            if composite.peek().and_then(|cwr| Some(cwr.0)) > Some(wr.0) {
                composite.push(WeightedRange::new(cp*2, cp));
                true
            }
            else {
                match composite.peek_mut() {
                    None => (),
                    Some(mut temp_wr) => { temp_wr.deref_mut().next_mut() }
                };
                false
            }
        },
        // Number is prime
        Some(wr) if wr.0 > cp => { 
            let mut new_wr = WeightedRange::new(cp, cp);
            new_wr.next_mut();
            composite.push(new_wr); true 
            },
        // Something is wrong.
        Some(_) => false
    }
}

fn main() {
    let mut composite: BinaryHeap<WeightedRange> = BinaryHeap::with_capacity(1000);
    //composite.push(WeightedRange::new(11*11, 11));
    let candidates = SteppingCounter::new_from(11, Ring2357::new())
        .filter(|cp| filter_prime(*cp, &mut composite))
        .take(99_997)
        .collect::<Vec<i32>>();
    let mut primes = vec![2,3,5,7];
    primes.extend(candidates);
    println!("{}: {}", primes.len(), primes.last().unwrap_or(&-1));
}

#[bench]
fn bench_primes_10000(b: &mut test::Bencher) {
    b.iter(|| {
        let mut composite: BinaryHeap<WeightedRange> = BinaryHeap::with_capacity(100000);
        let candidates = SteppingCounter::new_from(11, Ring2357::new())
            .filter(|cp| filter_prime(*cp, &mut composite))
            .take(9_997)
            .collect::<Vec<i32>>();
    });
}

#[bench]
fn bench_primes_100000(b: &mut test::Bencher) {
    b.iter(|| {
        let mut composite: BinaryHeap<WeightedRange> = BinaryHeap::with_capacity(100000);
        let candidates = SteppingCounter::new_from(11, Ring2357::new())
            .filter(|cp| filter_prime(*cp, &mut composite))
            .take(99_997)
            .collect::<Vec<i32>>();
    });
}

fn bench_primes_simple_10000(b: &mut test::Bencher) {
    b.iter(|| {
        let mut composite: BinaryHeap<WeightedRange> = BinaryHeap::with_capacity(100000);
        let candidates = (3..).step_by(2)
            .filter(|cp| filter_prime(*cp, &mut composite))
            .take(9_997)
            .collect::<Vec<i32>>();
    });
}

fn bench_primes_simple_100000(b: &mut test::Bencher) {
    b.iter(|| {
        let mut composite: BinaryHeap<WeightedRange> = BinaryHeap::with_capacity(100000);
        composite.push(WeightedRange::new(11*11, 11));
        let candidates = (3..).step_by(2)
            .filter(|cp| filter_prime(*cp, &mut composite))
            .take(99_997)
            .collect::<Vec<i32>>();
    });
}

