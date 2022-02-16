mod levenshtein;


#[test]
fn test_levenshtein() {
    let res = levenshtein::levenshtein(&String::from("sa11a"), &String::from("saa1"));

    println!("{:#?}", res)
}

#[test]
fn test_token_set() {
    let res = levenshtein::token_set_ratio(&String::from(" Zebra A good day"), &String::from("A bad zebra day" ));

    println!("{:#?}", res)
}


#[test]
fn test_ratio() {
    let res = levenshtein::lev_ratio(&String::from("Zxtfwu:?1!"), &String::from("uxa"));

    println!("{:#?}", res)
}


fn main() {
    levenshtein::levenshtein(&String::from("saa"), &String::from("safffa"));
}
