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
        xy: [(usize, isize); n],
    }

    let mut dp: Vec<(isize, isize)> = vec![(std::isize::MIN, std::isize::MIN); n + 1];
    dp[0] = (0, 0);

    for (i, &t) in xy.iter().enumerate() {
        if t.0 == 0 {
            dp[i + 1].0 = max(dp[i].0, max(dp[i].0, dp[i].1) + t.1);
        } else {
            dp[i + 1].1 = max(dp[i].1, dp[i].0 + t.1);
        }

        dp[i + 1].0 = max(dp[i].0, dp[i + 1].0);
        dp[i + 1].1 = max(dp[i].1, dp[i + 1].1);
    }

    println!("{}", max(dp[n].0, dp[n].1));
}
