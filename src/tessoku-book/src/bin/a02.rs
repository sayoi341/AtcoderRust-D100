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
        x: usize,
        a: [usize; n]
    }

    for i in 0..n {
        if a[i] == x {
            println!("Yes");
            process::exit(0);
        }
    }

    println!("No");
}
