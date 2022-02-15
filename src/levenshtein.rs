use std::cmp::min;

pub fn levenshtein(a_str: &String, b_str: &String) -> u32 {
    let a = a_str.as_bytes();
    let b= b_str.as_bytes();

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
