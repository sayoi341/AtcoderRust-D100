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
        mut n: usize,
    }

    let mut ans = vec!['0'; 10];

    for i in 0..10 {
        if n >= 2_usize.pow(9 - i as u32) {
            ans[i] = '1';
            n -= 2_usize.pow(9 - i as u32);
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
