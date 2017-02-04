#![feature(test)]

extern crate test;

fn main() {
    let abcs = (335..999).flat_map(|c| {
        let min = (1000 - c) / 2 + 1;
        let max = if c <= 500 { c } else { 1000 - c + 1 };
        (min..max).map(move |b| {
            let a = 1000 - b - c;
            (a, b, c)
        })
        .filter(|abc| abc.0 * abc.0 + abc.1 * abc.1 == abc.2 * abc.2)
        .collect::<Vec<(i32,i32,i32)>>()
    })
    .take(1)
    ;

    for (a,b,c) in abcs {
        println!("{:3} x {:3} x {:3} = {}", c, b, a, a*b*c);
    }
}

#[bench]
fn test_one_early_filter(b: &mut test::Bencher) {
    let mut dummy = (0,0,0);
    b.iter(|| {
        let ans = (335..999).flat_map(|c| {
            let min = (1000 - c) / 2 + 1;
            let max = if c <= 500 { c } else { 1000 - c + 1 };
        
            (min..max).map(move |b| {
                let a = 1000 - b - c;
                //println!("{}, {}, {}", a, b, c);
                (a, b, c)
            })
            .filter(|abc| abc.0 * abc.0 + abc.1 * abc.1 == abc.2 * abc.2)
            .collect::<Vec<(i32,i32,i32)>>()
        })
        .take(1).last();
        match ans {
            Some((a, b, c)) => dummy = (a,b,c),
            None => println!("No answer")
        }
    })
}

#[bench]
fn test_one(b: &mut test::Bencher) {
    let mut dummy = (0,0,0);
    b.iter(|| {
        let ans = (335..999).flat_map(|c| {
            let min = (1000 - c) / 2 + 1;
            let max = if c <= 500 { c } else { 1000 - c + 1 };
        
            (min..max).map(move |b| {
                let a = 1000 - b - c;
                //println!("{}, {}, {}", a, b, c);
                (a, b, c)
            }).collect::<Vec<(i32,i32,i32)>>()
        })
        .filter(|abc| abc.0 * abc.0 + abc.1 * abc.1 == abc.2 * abc.2)
        .take(1).last();
        match ans {
            Some((a, b, c)) => dummy = (a,b,c),
            None => println!("No answer")
        }
    })
}

#[bench]
fn test_all(b: &mut test::Bencher) {
    let mut dummy = (0,0,0);
    b.iter(|| {
        let ans = (335..999).flat_map(|c| {
            let min = (1000 - c) / 2 + 1;
            let max = if c <= 500 { c } else { 1000 - c + 1 };
        
            (min..max).map(move |b| {
                let a = 1000 - b - c;
                //println!("{}, {}, {}", a, b, c);
                (a, b, c)
            }).collect::<Vec<(i32,i32,i32)>>()
        })
        .filter(|abc| abc.0 * abc.0 + abc.1 * abc.1 == abc.2 * abc.2);
        for (a,b,c) in ans {
            dummy = (a,b,c);
        }
    })
}
