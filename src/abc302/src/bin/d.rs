#![allow(unused_imports)]
use itertools::Itertools;
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

// ABC302-D

fn main() {
    input! {
        n: usize,
        m: usize,
        d: isize,
        mut aa: [isize; n],
        mut ba: [isize; m],
    }

    aa.sort();
    ba.sort();

    let mut ans = -1;

    for a in aa {
        let bi = match ba.binary_search_by(|b| b.cmp(&(a + d))) {
            Ok(i) => i,
            Err(i) => {
                if i == 0 {
                    i
                } else {
                    i - 1
                }
            }
        };

        if (a - ba[bi]).abs() <= d {
            ans = max(ans, a + ba[bi]);
        }
    }

    println!("{}", ans);
}
