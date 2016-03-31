#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

use std::io;

fn calc(b: usize, n: usize, m: &Vec<usize>) -> usize {
    let n = n - 1;
    let mut min = 100000;
    let mut max = 1;
    for &x in m {
        if x < min {
            min = x;
        }
        if x > max {
            max = x;
        }
    }
    min *= n / b;
    max *= n / b + 1;

    let mut ready = vec![];
    'outer: loop {
        let mid = (min + max) / 2;
        ready.clear();
        let mut prev = 0;
        for (i, x) in m.iter().enumerate() {
            let done = mid / x;
            let doing = if mid % x > 0 {
                1
            } else {
                0
            };
            prev += done + doing;
            if prev > n {
                max = mid;
                continue 'outer;
            }
            if doing == 0 {
                ready.push(i + 1);
            }
        }
        let batch = (n - prev) as usize;
        if batch < ready.len() {
            return ready[batch];
        }
        min = mid;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut test_cases = String::new();
    stdin.read_line(&mut test_cases).unwrap();
    let test_cases: u32 = test_cases.trim().parse().unwrap();

    let mut case = String::new();
    for t in 0..test_cases {
        stdin.read_line(&mut case).unwrap();
        let (b, n) = {
            let mut line = case.trim().split_whitespace().map(|n| n.parse::<usize>().unwrap());
            (line.next().unwrap(), line.next().unwrap())
        };
        case.clear();
        stdin.read_line(&mut case).unwrap();
        let result = calc(b,
                          n,
                          &case.trim()
                               .split_whitespace()
                               .map(|n| n.parse::<usize>().unwrap())
                               .collect::<Vec<_>>());
        case.clear();
        println!("Case #{}: {}", t + 1, result);
    }
}
