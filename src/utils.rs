pub fn join_str(vec: Vec<&&str>) -> String {
    let mut ret = String::from("");

    for i in 0..vec.len() {
        ret += " ";
        ret += vec[i];
    }

    let ret_trim = ret.trim();
    return ret_trim.to_string()
}

pub fn join_space(a: &String, b: &String) -> String {
    let ret = a.to_owned() + " " + b;

    return ret.trim().to_string()
}

pub fn return_larger_smaller<'a>(a: &'a String, b: &'a String) -> (&'a String, &'a String) {
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
