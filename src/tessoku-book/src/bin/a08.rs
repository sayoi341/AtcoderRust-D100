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
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    let mut sum = vec![vec![0; w + 1]; h + 1];

    for i in 0..h {
        for j in 0..w {
            sum[i + 1][j + 1] = sum[i + 1][j] + sum[i][j + 1] - sum[i][j] + x[i][j];
        }
    }

    for i in 0..q {
        println!(
            "{}",
            sum[abcd[i].2][abcd[i].3]
                - sum[abcd[i].0 - 1][abcd[i].3]
                - sum[abcd[i].2][abcd[i].1 - 1]
                + sum[abcd[i].0 - 1][abcd[i].1 - 1]
        );
    }
}
