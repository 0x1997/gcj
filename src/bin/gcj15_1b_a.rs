#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

use std::io;

fn reverse(n: usize) -> usize {
    let mut input = n;
    let mut output = 0;
    while input > 0 {
        output = output * 10 + input % 10;
        input /= 10;
    }
    output
}

fn calc_cache(cache: &mut [(usize, usize)]) {
    for i in 1..14usize {
        let width = i as u32 + 1;
        let half = width / 2;
        let x = (10usize.pow(half - 1) + 1) * 10usize.pow(width - half) - 1;
        let y = reverse(x);
        let prev = cache[i - 1];
        let cnt = prev.1 + x - prev.0 + 1;
        cache[i] = (y, cnt);
    }
}

fn calc(mut n: usize, cache: &[(usize, usize)]) -> usize {
    let mut sum = 0;
    loop {
        if n <= 19 {
            return sum + n;
        }
        if n % 10 == 0 {
            sum += 1;
            n -= 1;
            continue;
        }
        let width = n.to_string().as_bytes().len() as u32;
        let half = width / 2;
        let right = n % 10usize.pow(width - half);
        n -= right - 1;
        sum += right - 1;

        let rev = reverse(n);
        if n > rev {
            sum += 1;
            n = rev;
            continue;
        }
        assert!(n == rev);
        let (y, cnt) = cache[width as usize - 2];
        return n - y + cnt + sum;
    }
}

fn main() {
    let stdin = io::stdin();
    let mut test_cases = String::new();
    stdin.read_line(&mut test_cases).unwrap();
    let test_cases: usize = test_cases.trim().parse().unwrap();

    let mut cache = [(0, 0); 14];
    calc_cache(&mut cache[..]);
    let mut case = String::new();
    for t in 0..test_cases {
        stdin.read_line(&mut case).unwrap();
        let result = calc(case.trim().parse::<usize>().unwrap(), &cache);
        case.clear();
        println!("Case #{}: {}", t + 1, result);
    }
}
