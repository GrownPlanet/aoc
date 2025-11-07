use core::str;
use std::collections::HashMap;

use aoc_helper::read_file;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let (towels, designs) = input.split_once("\n\n").unwrap();
    let towels: Vec<&str> = towels.split(", ").collect();

    let mut memo = HashMap::new();

    let mut result = 0;
    for design in designs.lines() {
        if is_posssible(design, &towels, &mut memo) {
            result += 1;
        }
    }
    println!("{}", result);
}

fn is_posssible<'a>(design: &'a str, towels: &Vec<&str>, memo: &mut HashMap<&'a str, bool>) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(b) = memo.get(design) {
        return *b;
    }

    for towel in towels {
        if begins_with(design, towel) {
            if is_posssible(
                str::from_utf8(&design.as_bytes()[towel.len()..]).unwrap(),
                towels,
                memo
            ) {
                memo.insert(design, true);
                return true;
            }
        }
    }

    memo.insert(design, false);
    return false;
}

fn begins_with(s1: &str, s2: &str) -> bool {
    if s1.len() < s2.len() {
        return false;
    }

    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();

    for i in 0..b2.len() {
        if b1[i] != b2[i] {
            return false;
        }
    }

    return true;
}
