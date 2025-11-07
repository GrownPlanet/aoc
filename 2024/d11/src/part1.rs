use std::fs;

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    
    let mut stones: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    
    for _ in 0..25 {
        stones = blink(&stones);
    }

    println!("{}", stones.len());
}

fn blink(stones: &Vec<i64>) -> Vec<i64> {
    let mut new_stones = vec![];

    for &stone in stones {
        if stone == 0 {
            new_stones.push(1);
            continue; }     

        let str_stone = stone.to_string();
        if str_stone.len() % 2 == 0 {
            let (s1, s2) = str_stone.split_at(str_stone.len() / 2);
            new_stones.push(s1.parse::<i64>().unwrap());
            new_stones.push(s2.parse::<i64>().unwrap());
        }
        else {
            new_stones.push(stone * 2024);
        }
    }

    new_stones
}
