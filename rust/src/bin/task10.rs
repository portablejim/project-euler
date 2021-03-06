use std::collections::BinaryHeap;
use std::cmp::Ordering;

// From functional infinite sieve paper
// https://www.cs.hmc.edu/~oneill/papers/Sieve-JFP.pdf
const RING2357: &'static [i32; 48] = &[2,4,2,4,6,2,6,4,2,4,6,6,2,6,4,2,6,4,6,8,4,2,4,2,4,8,6,4,6,2,4,6,2,6,6,4,2,4,6,2,6,4,2,4,2,10,2,10];

struct Ring2357 {
    offset: usize,
}

impl Ring2357 {
    fn new() -> Ring2357 {
        Ring2357 { offset: RING2357.len()-1 }
    }
}

impl Iterator for Ring2357 {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.offset = (self.offset + 1) % RING2357.len();
        //println!("Ring - {}", RING2357[self.offset]);
        Some(RING2357[self.offset])
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

    fn new_from_to(start: i32, end: i32, iterator: Ring2357) -> SteppingCounter {
        SteppingCounter { current: start, maximum: end, it: iterator }
    }
}

impl Iterator for SteppingCounter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        match self.it.next() {
            None => None,
            Some(x) if x > self.maximum => None,
            Some(x) => match x { 
                n if n + self.current > self.maximum => None,
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
        None => { &mut composite.push(WeightedRange::new(cp, cp).next()); true },
        // Number is composite, so go to the next one.
        Some(wr) if wr.0 == cp => { 
            while composite.peek().and_then(|cwr| Some(cwr.0)) == Some(wr.0) {
                match composite.pop() {
                    None => (),
                    Some(temp_wr) => { &mut composite.push(temp_wr.next()); () }
                };
            };
            false
        },
        Some(wr) if wr.0 < cp => { 
            while composite.peek().and_then(|cwr| Some(cwr.0)) <= Some(wr.0) {
                match composite.pop() {
                    None => (),
                    Some(temp_wr) => { &mut composite.push(temp_wr.next()); () }
                };
            };
            filter_prime(cp, composite)
        },
        // Number is prime
        Some(wr) if wr.0 > cp => { composite.push(WeightedRange::new(cp*2, cp)); true },
        // Something is wrong.
        Some(_) => false //panic!("Somehow the composite table of {} ({}) is less than the running total of {}", wr.0, wr.1, cp)
    }
}

fn main() {
    let mut composite: BinaryHeap<WeightedRange> = BinaryHeap::with_capacity(1000);
    composite.push(WeightedRange::new(11*11, 11));
    let prime_sum_part: i64 = SteppingCounter::new_from_to(11, 2_000_000, Ring2357::new())
        .filter(|cp| filter_prime(*cp, &mut composite))
        .map(|n| { n as i64 } )
        .sum();
    let prime_sum = 2 + 3 + 5 + 7 + prime_sum_part;
    println!("{}", prime_sum);
}