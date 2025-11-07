use std::collections::HashMap;
use std::fs;

pub fn part2() {
    let raw_input = fs::read_to_string("input.txt").unwrap();
    // let raw_input = fs::read_to_string("test_input.txt").unwrap();
    let input = raw_input.trim();

    let mut list_1 = vec![];
    let mut list_2 = HashMap::new();

    for l in input.lines() {
        let splt: Vec<&str> = l.split_whitespace().collect();
        list_1.push(splt[0].parse::<i32>().unwrap());
        let n2 = splt[1].parse::<i32>().unwrap();
        match list_2.get(&n2) {
            Some(n) => list_2.insert(n2, n + 1),
            None => list_2.insert(n2, 1),
        };
    }

    let mut res = 0;
    for n in list_1.iter() {
        let mult = list_2.get(n).unwrap_or(&0);
        res += n * mult;
    }

    println!("{}", res);
}
