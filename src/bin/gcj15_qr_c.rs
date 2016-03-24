#![feature(iter_arith)]
#![feature(zero_one)]

use std::cmp::min;
use std::fmt;
use std::io;
use std::num::One;
use std::ops::{Mul, Neg};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Q {
    O(i8),
    I(i8),
    J(i8),
    K(i8),
}

impl Q {
    fn new(ch: u8) -> Q {
        match ch {
            b'i' => Q::I(1),
            b'j' => Q::J(1),
            b'k' => Q::K(1),
            _ => unreachable!("Unkown character: {}", ch),
        }
    }
}

impl fmt::Display for Q {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (sign, ch) = match *self {
            Q::O(s) => (s, '1'),
            Q::I(s) => (s, 'i'),
            Q::J(s) => (s, 'j'),
            Q::K(s) => (s, 'k'),
        };
        write!(f,
               "{}{}",
               if sign > 0 {
                   ""
               } else {
                   "-"
               },
               ch)
    }
}

impl One for Q {
    fn one() -> Q {
        Q::O(1)
    }
}

impl Neg for Q {
    type Output = Q;

    fn neg(self) -> Q {
        match self {
            Q::O(s) => Q::O(-s),
            Q::I(s) => Q::I(-s),
            Q::J(s) => Q::J(-s),
            Q::K(s) => Q::K(-s),
        }
    }
}

impl Mul for Q {
    type Output = Q;

    fn mul(self, rhs: Q) -> Q {
        match (self, rhs) {
            (Q::O(sa), Q::O(sb)) => Q::O(sa * sb),
            (Q::O(sa), Q::I(sb)) => Q::I(sa * sb),
            (Q::O(sa), Q::J(sb)) => Q::J(sa * sb),
            (Q::O(sa), Q::K(sb)) => Q::K(sa * sb),

            (Q::I(sa), Q::O(sb)) => Q::I(sa * sb),
            (Q::I(sa), Q::I(sb)) => Q::O(-sa * sb),
            (Q::I(sa), Q::J(sb)) => Q::K(sa * sb),
            (Q::I(sa), Q::K(sb)) => Q::J(-sa * sb),

            (Q::J(sa), Q::O(sb)) => Q::J(sa * sb),
            (Q::J(sa), Q::I(sb)) => Q::K(-sa * sb),
            (Q::J(sa), Q::J(sb)) => Q::O(-sa * sb),
            (Q::J(sa), Q::K(sb)) => Q::I(sa * sb),

            (Q::K(sa), Q::O(sb)) => Q::K(sa * sb),
            (Q::K(sa), Q::I(sb)) => Q::J(sa * sb),
            (Q::K(sa), Q::J(sb)) => Q::I(-sa * sb),
            (Q::K(sa), Q::K(sb)) => Q::O(-sa * sb),
        }
    }
}

struct Input<'a> {
    line: &'a [u8],
    i: usize,
    size: usize,
}

impl<'a> Input<'a> {
    fn new(l: i64, x: i64, line: &[u8]) -> Input {
        Input {
            line: line,
            i: 0,
            size: (l * x) as usize,
        }
    }
}

impl<'a> Iterator for Input<'a> {
    type Item = Q;

    fn next(&mut self) -> Option<Q> {
        if self.i < self.size {
            let i = self.i % self.line.len();
            self.i += 1;
            Some(Q::new(self.line[i]))
        } else {
            None
        }
    }
}

fn calc(l: i64, x: i64, line: &str) -> bool {
    if !match Input::new(l, 1, line.as_bytes()).product() {
        Q::O(s) => s == -1 && x % 2 == 1,
        _ => x % 4 == 2,
    } {
        return false;
    }

    let mut expecting = Q::I(1);
    let mut p = Q::O(1);
    for ch in Input::new(l, min(8, x), line.as_bytes()) {
        p = p * ch;
        if p == expecting {
            expecting = match p {
                Q::I(1) => Q::J(1),
                Q::J(1) => return true,
                _ => unreachable!("Bug: expecting={}", p),
            };
            p = Q::O(1);
        }
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
        let (l, x) = {
            let mut nums = case.trim().split_whitespace().map(|n| n.parse::<i64>().unwrap());
            (nums.next().unwrap(), nums.next().unwrap())
        };
        case.clear();
        stdin.read_line(&mut case).unwrap();
        let result = calc(l, x, case.trim());
        case.clear();
        println!("Case #{}: {}",
                 t + 1,
                 if result {
                     "YES"
                 } else {
                     "NO"
                 });
    }
}
