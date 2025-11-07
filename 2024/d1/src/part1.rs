use std::fs;

pub fn part1() {
    // let raw_input = fs::read_to_string("input.txt").unwrap();
    let raw_input = fs::read_to_string("test_input.txt").unwrap();
    let input = raw_input.trim();

    let mut list_1 = vec![];
    let mut list_2 = vec![];

    for l in input.lines() {
        let splt: Vec<&str> = l.split_whitespace().collect();
        list_1.push(splt[0].parse::<i32>().unwrap());
        list_2.push(splt[1].parse::<i32>().unwrap());
    }

    list_1.sort();
    list_2.sort();

    let mut sum = 0;
    for (i, v1) in list_1.iter().enumerate() {
        let v2 = list_2[i];
        sum += (v1 - v2).abs();
    }

    println!("{}", sum);
}
