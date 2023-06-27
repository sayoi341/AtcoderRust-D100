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

// ABC304-D

fn main() {
    input! {
        _w: usize,
        _h: usize,
        n: usize,
        pq: [(usize, usize); n],
        na: usize,
        a: [usize; na],
        nb: usize,
        b: [usize; nb],
    }

    let mut map: BTreeMap<(usize, usize), usize> = BTreeMap::new();

    for (p, q) in pq {
        let x = a.binary_search(&p).unwrap_err();
        let y = b.binary_search(&q).unwrap_err();
        map.entry((x, y)).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut vec = map.into_iter().map(|(_, v)| v).collect::<Vec<usize>>();

    if vec.len() < (na + 1) * (nb + 1) {
        vec.push(0);
    }

    println!(
        "{} {}",
        vec.iter().min().unwrap(),
        vec.iter().max().unwrap()
    );
}
