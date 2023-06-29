#![allow(unused_imports)]
use itertools::Itertools;
use libm::sqrt;
use proconio::{input, marker::*};
use std::{
    cmp::{max, min},
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
    io::*,
    iter::{FromIterator, IntoIterator},
    ops::{Add, BitAnd, Div, Mul, Neg, Sub},
    process,
    str::FromStr,
    vec,
};

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = 0;

    for i in 1..=n {
        for j in 1..=n {
            let l: isize = k as isize - i as isize - j as isize;
            if 1 <= l && l <= n as isize {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
