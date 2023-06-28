#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{input, marker::*};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    io::*,
    iter::{FromIterator, IntoIterator},
    ops::{Add, BitAnd, Div, Mul, Neg, Sub},
    str::FromStr,
    vec,
};

// ABC301-D

fn main() {
    input! {
        s: Chars,
        n: usize,
    }

    let mut ans: isize = 0;

    for (i, c) in s.iter().enumerate() {
        if *c == '1' {
            ans += 2isize.pow((s.len() - i - 1) as u32);
        } else if *c == '?' {
            let mut tmp = ans + 2isize.pow((s.len() - i - 1) as u32);

            for j in i + 1..s.len() {
                if s[j] == '1' {
                    tmp += 2isize.pow((s.len() - j - 1) as u32);
                }
            }

            if tmp <= n as isize {
                ans += 2isize.pow((s.len() - i - 1) as u32);
            }
        }
    }

    println!("{}", if ans <= n as isize { ans } else { -1 });
}
