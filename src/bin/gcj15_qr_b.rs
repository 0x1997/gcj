use std::cmp::min;
use std::io;

fn calc(plates: &mut Vec<i32>) -> i32 {
    let max_pancakes = *plates.iter().max().unwrap();
    (1..max_pancakes).fold(max_pancakes, |minutes, x| {
        let moves: i32 = plates.iter()
            .map(|plate| (plate - 1) / x)
            .sum();
        min(minutes, x + moves)
    })
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
        let minutes = calc(&mut case.trim()
            .split_whitespace()
            .map(|plate| plate.parse::<i32>().unwrap())
            .collect::<Vec<i32>>());
        case.clear();
        println!("Case #{}: {}", t + 1, minutes);
    }
}
