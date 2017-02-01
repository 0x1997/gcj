use std::io;

fn calc(max_shyness: i32, audience: &str) -> i32 {
    let mut friends = 0;
    let mut standing = 0;
    for (shyness, digit) in (0..max_shyness + 1).zip(audience.chars()) {
        let num = digit.to_digit(10).unwrap() as i32;
        if num == 0 {
            continue;
        }
        if shyness > standing {
            let new_friends = shyness - standing;
            friends += new_friends;
            standing += new_friends;
        }
        standing += num;
    }
    friends
}

fn main() {
    let stdin = io::stdin();
    let mut test_cases = String::new();
    stdin.read_line(&mut test_cases).unwrap();
    let test_cases: u32 = test_cases.trim().parse().unwrap();

    let mut case = String::new();
    for t in 0..test_cases {
        stdin.read_line(&mut case).unwrap();
        let friends = {
            let mut iter = case.splitn(2, ' ');
            let max_shyness: i32 = iter.next().unwrap().parse().unwrap();
            calc(max_shyness, iter.next().unwrap())
        };
        case.clear();
        println!("Case #{}: {}", t + 1, friends);
    }
}
