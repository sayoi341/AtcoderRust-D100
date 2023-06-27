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

// ABC305 D

fn main() {
    input! {
        n: usize,
        a:[usize;n],
        q: usize,
        lr: [(usize,usize);q]
    }

    let mut sum = vec![0; n];

    for i in 1..n {
        sum[i] = sum[i - 1];
        if i % 2 == 0 {
            sum[i] += a[i] - a[i - 1];
        }
    }

    for (l, r) in lr {
        let mut ans = 0;

        let il = a.binary_search(&l).unwrap_or_else(|x| x);
        let ir = a.binary_search(&r).unwrap_or_else(|x| x);

        ans += sum[ir] - sum[il];

        if il % 2 == 0 {
            ans += a[il] - l;
        }

        if ir % 2 == 0 {
            ans -= a[ir] - r;
        }

        println!("{}", ans);
    }
}
