use std::io;

fn calc(x: i32, r: i32, c: i32) -> bool {
    // R: hollow
    if x >= 7 {
        return true;
    }
    // plain:
    let p = r * c;
    // G: cannot fill
    if p % x != 0 {
        return true;
    }
    let (g_l, g_s) = if r > c {
        (r, c)
    } else {
        (c, r)
    };
    // R: can do nothing
    if x <= g_s {
        return false;
    }
    // R: strech R x C
    if x > g_l || x / 2 + x % 2 > g_s {
        return true;
    }
    // R: hole
    if x > g_s * 2 - 1 + 1 {
        return true;
    }
    // R: corner
    if (1..g_s).any(|h| {
        let len = x - g_s + 1;
        len >= g_s &&
        (0..(g_l - len)).all(|n| {
            let h1 = g_s * (n + 1) + h;
            let h2 = p - h1 - x;
            h1 % x != 0 && h2 % x != 0
        })
    }) {
        return true;
    }
    false
}

fn main() {
    let stdin = io::stdin();
    let mut test_cases = String::new();
    stdin.read_line(&mut test_cases).unwrap();
    let test_cases: u32 = test_cases.trim().parse().unwrap();

    let mut case = String::new();
    for t in 0..test_cases {
        stdin.read_line(&mut case).unwrap();
        let result = {
            let mut nums = case.trim().split_whitespace().map(|n| n.parse::<i32>().unwrap());
            calc(nums.next().unwrap(),
                 nums.next().unwrap(),
                 nums.next().unwrap())
        };
        case.clear();
        println!("Case #{}: {}",
                 t + 1,
                 if result {
                     "RICHARD"
                 } else {
                     "GABRIEL"
                 });
    }
}
