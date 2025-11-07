use std::{collections::HashMap, fs};

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    
    let stones: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut mem = HashMap::new();

    let mut result = 0;
    for stone in stones {
        result += splits_to(stone, 75, &mut mem);
    }

    println!("{}", result);
}

fn splits_to(stone: i64, remaining: u8, mem: &mut HashMap<(i64, u8), i64>) -> i64 {
    if remaining == 0 {
        return 1;
    }

    if let Some(ret) = mem.get(&(stone, remaining)) {
        return *ret;
    }

    if stone == 0 {
        let ret = splits_to(1, remaining - 1, mem);
        mem.insert((stone, remaining), ret);
        return ret;
    }

    let str_stone = stone.to_string();
    if str_stone.len() % 2 == 0 {
        let (s1, s2) = str_stone.split_at(str_stone.len() / 2);
        let ret = splits_to(s1.parse::<i64>().unwrap(), remaining - 1, mem)
               + splits_to(s2.parse::<i64>().unwrap(), remaining - 1, mem);
        mem.insert((stone, remaining), ret);
        return ret;
    }
    else {
        let ret = splits_to(stone * 2024, remaining - 1, mem);
        mem.insert((stone, remaining), ret);
        return ret;
    }
}
