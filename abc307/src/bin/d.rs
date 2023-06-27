#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    io::*,
    iter::{FromIterator, IntoIterator},
    ops::{Add, Div, Mul, Neg, Sub},
    str::FromStr,
    vec,
};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut cs: Vec<char> = vec![];
    let mut cnt = 0;

    for i in 0..n {
        if s[i] == '(' {
            cnt += 1;
        }

        cs.push(s[i]);

        if s[i] == ')' {
            if cnt != 0 {
                loop {
                    let c = cs.pop();
                    if c == Some('(') {
                        cnt -= 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{}", cs.iter().collect::<String>());
}
