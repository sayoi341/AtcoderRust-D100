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
        a: [usize; n],
    }

    let mut vans = vec![0; n];

    for i in 0..n {
        if i == 0 {
            vans[i] = 1;
        } else {
            vans[i] = vans[i - 1];
        }

        while vans[i] < n && a[vans[i]] - a[i] <= k {
            vans[i] += 1;
        }
    }

    let mut ans = 0;

    for i in 0..n {
        ans += vans[i] - i - 1;
    }

    println!("{}", ans);
}
