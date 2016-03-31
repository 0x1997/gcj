#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate gcj;

use std::io;
use std::cmp::Ordering;

use gcj::heap;

#[derive(Debug)]
struct Hiker {
    time_to_end: usize,
    lap_time: usize,
    first_round: bool,
}

fn time_to_end(x: &Hiker, y: &Hiker) -> Ordering {
    (x.time_to_end, x.first_round).cmp(&(y.time_to_end, y.first_round))
}

fn calc(hikers: &mut [Hiker]) -> usize {
    let num_hikers = hikers.len();
    let mut min_encounters = num_hikers;
    let mut encounters = num_hikers;
    let mut slow_hikers = num_hikers;
    heap::heapify_by(hikers, &mut time_to_end);
    loop {
        {
            let first = &mut hikers[0];
            if first.first_round {
                encounters -= 1;
                slow_hikers -= 1;
                first.first_round = false;
            } else {
                encounters += 1;
            }
            if encounters < min_encounters {
                min_encounters = encounters;
            }
            if encounters >= min_encounters + slow_hikers {
                return min_encounters;
            }
            first.time_to_end += first.lap_time;
        }
        heap::fix_down_by(hikers, 0, &mut time_to_end);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut test_cases = String::new();
    stdin.read_line(&mut test_cases).unwrap();
    let test_cases: usize = test_cases.trim().parse().unwrap();

    let mut line = String::new();
    for t in 0..test_cases {
        stdin.read_line(&mut line).unwrap();
        let num_groups: usize = line.trim().parse().unwrap();
        line.clear();
        let mut hikers = vec![];
        for _ in 0..num_groups {
            stdin.read_line(&mut line).unwrap();
            let group = line.trim()
                            .split_whitespace()
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect::<Vec<_>>();
            line.clear();
            for i in 0..group[1] {
                let time_per_deg = group[2] + i;
                hikers.push(Hiker {
                    time_to_end: time_per_deg * (360 - group[0]),
                    lap_time: time_per_deg * 360,
                    first_round: true,
                });
            }
        }
        let result = calc(&mut hikers[..]);
        println!("Case #{}: {}", t + 1, result);
    }
}
