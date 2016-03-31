#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

use std::cmp::{max, min};
use std::io;

fn reduction(mut to_remove: usize, inner_cells: usize, edge_cells: usize) -> usize {
    let mut reduction = 0;
    let removed = min(to_remove, inner_cells);
    to_remove -= removed;
    reduction += removed * 4;
    let removed = min(to_remove, edge_cells);
    to_remove -= removed;
    reduction += removed * 3;
    if to_remove > 0 {
        reduction += to_remove * 2;
    }
    reduction
}

fn calc(r: usize, c: usize, n: usize) -> usize {
    if n <= (r * c + 1) / 2 {
        return 0;
    }
    let to_remove = r * c - n;
    match (r, c) {
        (1, x) | (x, 1) => {
            let unhappiness = x - 1;
            unhappiness - to_remove * 2
        }
        _ => {
            let border_cells = r + c - 2;
            let unhappiness = (r - 1) * c + (c - 1) * r;
            let reduced_unhappiness = if r % 2 == 1 && c % 2 == 1 {
                let plan1 = reduction(to_remove, (r - 2) * (c - 2) / 2 + 1, border_cells - 4);
                let plan2 = reduction(to_remove, (r - 2) * (c - 2) / 2, border_cells);
                max(plan1, plan2)
            } else {
                let inner_cells = (r - 2) * (c - 2) / 2;
                let edge_cells = border_cells - 2;
                reduction(to_remove, inner_cells, edge_cells)
            };
            unhappiness - reduced_unhappiness
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut test_cases = String::new();
    stdin.read_line(&mut test_cases).unwrap();
    let test_cases: usize = test_cases.trim().parse().unwrap();

    let mut case = String::new();
    for t in 0..test_cases {
        stdin.read_line(&mut case).unwrap();
        let input = case.trim()
                        .split_whitespace()
                        .map(|n| n.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
        case.clear();
        let result = calc(input[0], input[1], input[2]);
        println!("Case #{}: {}", t + 1, result);
    }
}
