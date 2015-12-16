use std::cmp::min;
use std::io;

fn calc(n: &Vec<u32>) -> (u32, u32) {
    let mut sum1 = 0;
    let mut prev = 0;
    let mut speed = 0;
    for &i in n {
        if i < prev {
            let x = prev - i;
            sum1 += x;
            if x > speed {
                speed = x;
            }
        }
        prev = i;
    }
    prev = 0;
    let mut sum2 = 0;
    for &i in n {
        sum2 += min(prev, speed);
        prev = i;
    }
    (sum1, sum2)
}

fn main() {
    let stdin = io::stdin();
    let mut test_cases = String::new();
    stdin.read_line(&mut test_cases).unwrap();
    let test_cases: u32 = test_cases.trim().parse().unwrap();

    let mut case = String::new();
    for t in 0..test_cases {
        stdin.read_line(&mut case).unwrap();
        case.clear();
        stdin.read_line(&mut case).unwrap();
        let result = calc(&case.trim()
                               .split_whitespace()
                               .map(|n| n.parse::<u32>().unwrap())
                               .collect::<Vec<u32>>());
        case.clear();
        println!("Case #{}: {} {}", t + 1, result.0, result.1);
    }
}
