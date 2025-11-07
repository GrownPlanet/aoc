use std::fs;

use regex::Regex;

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let result = re
        .captures_iter(&input)
        .map(|c| {
            let [n1, n2] = c.extract().1;
            (n1.parse::<i32>().unwrap(), n2.parse::<i32>().unwrap())
        })
        .fold(0, |acc, (n1, n2)| acc + n1 * n2);

    println!("{}", result);
}
