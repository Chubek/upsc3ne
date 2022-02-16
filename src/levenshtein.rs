#[path = "./utils.rs"] mod utils;

use std::cmp::{max, min};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;
use math::round::ceil;

pub fn levenshtein(a_str: &String, b_str: &String) -> u32 {
    let larger_smaller = utils::return_larger_smaller(a_str, b_str);

    let a = larger_smaller.1.as_bytes();
    let b = larger_smaller.0.as_bytes();

    let m = a.len();
    let n = b.len();

    let mut d = vec![vec![0u32; n]; m];


    for k in 1..n {
        d[0usize][k] = k as u32;
    }


    for j in 1..n {
        for i in 1..m {
            let mut sub_cost = 1u32;

            if a[i] == b[j] {
                sub_cost = 0u32;
            }

            d[i][j] = min(&d[i - 1usize][j] + &1u32, min(&d[i][j - 1usize]  + &1u32, &d[i - 1usize][j - 1usize]  + sub_cost));



        }

    }

    return d[m - 1usize][n - 1usize]
}

pub fn lev_ratio(a: &String, b: &String) -> u8 {
    let d = levenshtein(a, b);

    let m = a.len() as u32;
    let n = b.len() as u32;

    let max = max(m, n);

    let d_f64 = d as f64;
    let max_f64 = max as f64;

    let ratio = (d_f64 / max_f64) * 100f64;


    let ratio_ceil = ceil(ratio, 2);



    return ratio_ceil as u8
}


pub fn token_set_ratio(a: &String, b: &String) -> u8 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[.,/#!?$%\^&\*;:{}=\-_`~()\[\]]").unwrap();
        static ref RESPACE: Regex = Regex::new(r"\s+").unwrap();
    }

    let a_lower = a.to_lowercase();
    let b_lower = b.to_lowercase();
    let a_trim = a_lower.trim();
    let b_trim = b_lower.trim();

    let a_rep = RE.replace_all(a_trim, "");
    let b_rep = RE.replace_all(b_trim, "");

    let a_split = RESPACE.split(&a_rep);
    let b_split = RESPACE.split(&b_rep);

    let a_set: HashSet<&str> = HashSet::from_iter(a_split);
    let b_set: HashSet<&str> = HashSet::from_iter(b_split);

    let a_b_intersect = a_set.intersection(&b_set);
    let a_diff_b = a_set.difference(&b_set);
    let b_diff_a = b_set.difference(&a_set);

    let mut sect_str = a_b_intersect
        .into_iter()
        .collect::<Vec<_>>();
    sect_str.sort();

    let sect_diff_ab = a_diff_b
        .into_iter()
        .collect::<Vec<_>>();
    sect_str.sort();

    let sect_diff_ba = b_diff_a
        .into_iter()
        .collect::<Vec<_>>();
    sect_str.sort();

    let sect_joined = utils::join_str(sect_str);
    let diff_ab_join = utils::join_str(sect_diff_ab);
    let diff_ba_join = utils::join_str(sect_diff_ba);

    let sect_ab = utils::join_space(&sect_joined, &diff_ab_join);
    let sect_ba= utils::join_space(&sect_joined, &diff_ba_join);

    let ratio_sect_ab = lev_ratio(&sect_joined, &sect_ab);
    let ratio_sect_ba = lev_ratio(&sect_joined, &sect_ba);
    let ratio_ab_ab = lev_ratio(&sect_ab, &sect_ba);

    let max_ratio = max(ratio_ab_ab, max(ratio_sect_ba, ratio_sect_ab));

    println!("{} {} {}", ratio_sect_ab, ratio_sect_ba, ratio_ab_ab);

    return max_ratio
}
