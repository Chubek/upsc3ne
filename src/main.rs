use std::borrow::BorrowMut;
use crate::dtype::Profanity;

mod levenshtein;
mod dtype;

#[test]
fn test_levenshtein() {
    let res = levenshtein::levenshtein(&String::from("sa11a"), &String::from("saa1"));

    println!("{:#?}", res)
}

#[test]
fn test_token_set() {
    let res = levenshtein::token_set_ratio(&String::from("fuck dude"), &String::from("suck dude" ));

    println!("{:#?}", res)
}


#[test]
fn test_ratio() {
    let res = levenshtein::lev_ratio(&String::from("Zxtfwu:?1!"), &String::from("uxa"));

    println!("{:#?}", res)
}


#[test]
fn test_process() {
    let choices = vec!["Fucked", "Sucked", "Duck"];

    let res = levenshtein::process_token_set_ratio("I fucked that sucker", choices);

    println!("{:#?}", res)
}

#[test]
fn test_sort() {
    let mut choices = vec![Profanity {content: "aaa", distance: 10},
                       Profanity {content: "aaa", distance: 20}, Profanity {content: "aaa", distance: 5}];

    dtype::sort_profanity_vector(choices.borrow_mut());

    println!("{:#?}", choices)
}




fn main() {
    levenshtein::levenshtein(&String::from("saa"), &String::from("safffa"));
}
