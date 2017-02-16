
fn main() {
    for i in 200..1_000_010 {
        {
            let t = i*(i+1)/2;
            let mut count = 0;
            let max = (t as f64).sqrt().ceil() as u64;
            for d in 1..max {
                if t % d == 0 {
                    count += 2;
                }
            }
            if count >= 500 {
                println!("{} - {}", t, count);
                return;
            }
        }
    }
}