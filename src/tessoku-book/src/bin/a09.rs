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
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }

    let mut sum = vec![vec![0; w + 1]; h + 1];

    for i in 0..n {
        sum[abcd[i].0 - 1][abcd[i].1 - 1] += 1;
        sum[abcd[i].0 - 1][abcd[i].3] -= 1;
        sum[abcd[i].2][abcd[i].1 - 1] -= 1;
        sum[abcd[i].2][abcd[i].3] += 1;
    }

    for i in 0..h {
        for j in 0..w {
            sum[i][j + 1] += sum[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            sum[i + 1][j] += sum[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", sum[i][j]);
        }
        println!();
    }
}
