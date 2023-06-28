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
    str::FromStr,
    vec,
};

// ABC300-D

fn era(n: usize) -> Vec<bool> {
    let mut isprime = vec![true; n];
    isprime[0] = false;
    isprime[1] = false;
    for i in 2..n {
        if isprime[i] {
            let mut j = 2 * i;
            while j < n {
                isprime[j] = false;
                j += i;
            }
        }
    }

    isprime
}

fn main() {
    input! {
        n: usize
    }

    let max = 1100000;

    let isprime = era(max);

    let mut s = vec![0; max + 1];

    for i in 0..max {
        s[i + 1] = s[i] + isprime[i] as usize;
    }

    let mut ans = 0;

    let mut a = 1;
    while a * a * a * a * a <= n {
        if !isprime[a] {
            a += 1;
            continue;
        }

        let mut b = a + 1;

        while b * b * b <= n {
            if !isprime[b] {
                b += 1;
                continue;
            }

            let cmax = sqrt((n / a / a / b) as f64) as usize;

            if b + 1 <= cmax + 1 {
                ans += s[cmax + 1] - s[b + 1];
            }

            b += 1;
        }

        a += 1;
    }

    println!("{}", ans);
}
