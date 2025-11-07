use std::fs;

use regex::Regex;

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let global_pat = Regex::new(r"(mul\(\d+,\d+\))|(don't\(\)|do\(\))").unwrap();

    let mut enabled = true;

    let result: i32 = global_pat
        .captures_iter(&input)
        .map(|c| {
            let pat = c.extract::<1>().1[0];
            match pat {
                "do()" => {enabled = true; 0},
                "don't()" => {enabled = false; 0},
                _ => if enabled { extract_mul(pat) } else { 0 },
            }
        })
        .sum();

    println!("{:?}", result);
}

fn extract_mul(pat: &str) -> i32 {
    let (mut n1, mut n2) = pat.split_once(',').unwrap();

    n1 = &n1[4..n1.len()];
    n2 = &n2[0..n2.len() - 1];

    n1.parse::<i32>().unwrap() * n2.parse::<i32>().unwrap()
}
