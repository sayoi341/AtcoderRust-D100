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
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut day = vec![0; d + 1];

    for i in 0..n {
        day[lr[i].0] += 1;
        day[lr[i].1 + 1] -= 1;
    }

    for i in 0..d {
        day[i + 1] += day[i];
        println!("{}", day[i + 1]);
    }
}
