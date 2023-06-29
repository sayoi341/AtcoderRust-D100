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
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    }

    let mut r = vec![0; n + 1];
    let mut l = vec![0; n + 1];

    r[0] = a[0];
    for i in 0..n {
        r[i + 1] = max(r[i], a[i]);
    }

    l[n] = a[n - 1];
    for i in (0..n).rev() {
        l[i] = max(l[i + 1], a[i]);
    }

    for i in 0..d {
        println!("{}", max(r[lr[i].0 - 1], l[lr[i].1]));
    }
}
