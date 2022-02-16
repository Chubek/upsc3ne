use std::cmp::{max, min};
use regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashSet;

fn return_larger_smaller<'a>(a: &'a String, b: &'a String) -> (&'a String, &'a String) {
    let len_a = a.len() as i32;
    let len_b = b.len() as i32;

    let diff = len_a - len_b;

    let larger: &String;
    let smaller: &String;

    if diff >= 0 {
        larger = a;
        smaller = b;
    } else {
        larger = b;
        smaller = a;
    }

    return (larger, smaller)


}

pub fn levenshtein(a_str: &String, b_str: &String) -> u32 {
    let larger_smaller = return_larger_smaller(a_str, b_str);

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

    return d[m - 1usize][n - 1usize] + 1
}

pub fn lev_ratio(a: &String, b: &String) -> u32 {
    let d = levenshtein(a, b);

    let m = a.len() as u32;
    let n = b.len() as u32;

    let max = max(m, n);

    let ratio = (d as f32) / (max as f32) * 100f32;

    println!("{}", ratio);

    return ratio as u32
}


pub fn token_set_ratio(a: &String, b: &String) -> u32 {
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

    let mut sect_diff_ab = a_diff_b
        .into_iter()
        .collect::<Vec<_>>();
    sect_str.sort();

    let mut sect_diff_ba = b_diff_a
        .into_iter()
        .collect::<Vec<_>>();
    sect_str.sort();

    println!("{:#?}\n{:#?}\n{:#?}", sect_str, sect_diff_ab, sect_diff_ba);
    return 0
}
