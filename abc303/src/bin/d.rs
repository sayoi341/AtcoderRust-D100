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

// ABC303-D

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
        s: Chars,
    }

    let mut dp: Vec<(usize, usize)> = vec![(1e9 as usize, 1e9 as usize); s.len() + 1];
    dp[0].0 = 0;

    for (i, &c) in s.iter().enumerate() {
        if c == 'a' {
            dp[i + 1].0 = (dp[i].0 + x).min(dp[i].1 + x + z);
            dp[i + 1].1 = (dp[i].1 + y).min(dp[i].0 + y + z);
        } else {
            dp[i + 1].0 = (dp[i].0 + y).min(dp[i].1 + y + z);
            dp[i + 1].1 = (dp[i].1 + x).min(dp[i].0 + x + z);
        }
    }

    println!(
        "{}",
        min(dp.iter().last().unwrap().0, dp.iter().last().unwrap().1)
    );
}
