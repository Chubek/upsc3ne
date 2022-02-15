mod levenshtein;

#[test]
fn test_levenshtein() {
    let res = levenshtein::levenshtein(&String::from("sa11a"), &String::from("saa1"));

    println!("{:#?}", res)
}




fn main() {
    levenshtein::levenshtein(&String::from("saa"), &String::from("safffa"));
}
