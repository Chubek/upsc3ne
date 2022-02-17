use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::borrow::BorrowMut;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ProfanityResult{
    pub profanities: Vec<ProfanityRecord>,
    pub final_score: f64
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ProfanityRecord{
    pub profanity: String,
    pub category: String,
    pub score: f64
}

pub fn read_records() -> Result<Vec<ProfanityRecord>, Box<dyn Error>> {
    let file = File::open(Path::new("src/records.json"))?;
    let reader = BufReader::new(file);

    let records: Vec<ProfanityRecord> = serde_json::from_reader(reader)?;

    Ok(records)
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Profanity {
    pub content:  &'static str,
    pub distance: u8
}

fn partition((start_pass, end_pass): ( usize,  usize), vector: &mut Vec<Profanity>, neg_flag: bool) -> usize{
    let mut start = start_pass.clone();
    let mut end = end_pass.clone();
    let   pivot_index = start;
    let pivot = vector[pivot_index].to_owned();

    while start < end && !neg_flag {
        while start < vector.len()  && vector[start].distance <= pivot.distance {
            start += 1;
        }

        while vector[end].distance > pivot.distance {
            end -= 1;

        }

        if start < end {
            let end_el = vector[end].to_owned();
            let start_el = vector[start].to_owned();

            vector[start] = end_el;
            vector[end] = start_el;

        }


    }

    let pivot_el = vector[pivot_index].to_owned();
    let end_el = vector[end].to_owned();

    vector[end] = pivot_el;
    vector[pivot_index] = end_el;

    return end
}

fn qs_vec((start_pass, end_pass): ( usize, usize), vector: &mut Vec<Profanity>, neg_flag: bool) {
    let start = start_pass.clone();
    let end = end_pass.clone();

    if start < end && !neg_flag {
        let  p = partition((start, end), vector.borrow_mut(), neg_flag);
        let p_plus_one = p + 1;

        let  p_min_one: usize;
        let neg_flag: bool;

        if p == 0 {
            p_min_one = 0;
            neg_flag = true;
        } else {
            p_min_one = p - 1;
            neg_flag = false;
        }

        qs_vec((start, p_min_one), vector.borrow_mut(), neg_flag);

        qs_vec((p_plus_one, end), vector.borrow_mut(), neg_flag);

    }


}


pub fn sort_profanity_vector(vector: &mut Vec<Profanity>){
    let start = 0usize;
    let end = vector.len() - 1;

    qs_vec((start, end), vector.borrow_mut(), false);
}