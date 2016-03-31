#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

use std::cmp::Ordering::{Equal, Greater, Less};
use std::io;

#[derive(Debug)]
struct IntVec {
    x: i64,
    y: i64,
}

impl IntVec {
    fn new<T: Copy>(p1: &(T, T), p2: &(T, T)) -> IntVec
        where i64: From<T>
    {
        IntVec {
            x: i64::from(p2.0) - i64::from(p1.0),
            y: i64::from(p2.1) - i64::from(p1.1),
        }
    }
}

fn cross(a: &IntVec, b: &IntVec) -> i64 {
    a.x * b.y - a.y * b.x
}

fn quadrant_order(a: &IntVec) -> i32 {
    match (a.x.signum(), a.y.signum()) {
        (1, 0) => 0,
        (1, 1) => 1,
        (0, 1) => 2,
        (-1, 1) => 3,
        (-1, 0) => 4,
        (-1, -1) => 5,
        (0, -1) => 6,
        (1, -1) => 7,
        _ => unreachable!(),
    }
}

fn calc(points: &Vec<(i32, i32)>) {
    let len = points.len();
    if len <= 3 {
        for _ in points {
            println!("0");
        }
        return;
    }
    for i in points {
        let mut surround: Vec<IntVec> = points.iter()
                                              .filter(|&j| *i != *j)
                                              .map(|j| IntVec::new(i, j))
                                              .collect();
        surround.sort_by(|a, b| {
            match quadrant_order(a).cmp(&quadrant_order(b)) {
                Equal => {
                    match cross(a, b).signum() {
                        1 => Less,
                        0 => Equal,
                        -1 => Greater,
                        _ => unreachable!(),
                    }
                }
                ord @ _ => ord,
            }
        });
        let surround = &surround;
        let len = surround.len();
        let mut min_cuts = len;
        let mut head = 0;
        for tail in 0..len {
            let vec_tail = &surround[tail];
            let mut head_eq_tail = head == tail;
            loop {
                let next_head = if head + 1 == len {
                    head_eq_tail = false;
                    0
                } else {
                    head + 1
                };
                if next_head == tail {
                    break;
                }
                let vec_next_head = &surround[next_head];
                let turn = cross(vec_tail, vec_next_head);
                if turn > 0 {
                    head = next_head;
                    head_eq_tail = false;
                } else if turn == 0 && head_eq_tail &&
                   quadrant_order(vec_tail) == quadrant_order(vec_next_head) {
                    head = next_head;
                } else {
                    break;
                }
            }
            let window_size = if head < tail {
                len - tail + head
            } else {
                head - tail
            };
            if window_size < min_cuts {
                min_cuts = window_size;
                if min_cuts == 0 {
                    break;
                }
            }

            if head == tail {
                head += 1;
            }
        }
        println!("{}", min_cuts);
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
        let num_trees: u32 = case.trim().parse().unwrap();
        case.clear();
        let trees = (0..num_trees)
                        .map(|_| {
                            stdin.read_line(&mut case).unwrap();
                            let point = {
                                let mut iter = case.trim()
                                                   .split_whitespace()
                                                   .map(|n| n.parse::<i32>().unwrap());
                                (iter.next().unwrap(), iter.next().unwrap())
                            };
                            case.clear();
                            point
                        })
                        .collect::<Vec<_>>();
        println!("Case #{}:", t + 1);
        calc(&trees);
    }
}
