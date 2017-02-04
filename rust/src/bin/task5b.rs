use std::collections::BinaryHeap;
use std::cmp::Ordering;

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
        Some(_) => false
    }
}

fn main() {
    let mut composite: BinaryHeap<WeightedRange> = BinaryHeap::with_capacity(30);
    let result = (3..21)
        .filter(|cp| filter_prime(*cp, &mut composite))
        .map(|f| f * (20 / f))
        .map(|c| { println!("C2: {:?}", c); c as u64 })
        .product::<u64>() / 2;
    println!("Result: {}", result);
    //let mut primes = candidates;
    //println!("{}: {}", primes.len(), primes.last().unwrap_or(&-1));
}