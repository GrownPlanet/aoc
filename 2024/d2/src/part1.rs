use std::fs;

pub fn solve() {
    // let raw_input = fs::read_to_string("test_input.txt").unwrap();
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let input = raw_input.trim();

    let mut result = 0;

    'l: for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        for i in 0..nums.len() - 1 {
            let diff = nums[i] - nums[i + 1];
            if is_not_correct(diff, nums[0] - nums[1] > 0) {
                continue 'l;
            }
        }

        result += 1;
    }

    println!("{}", result);
}

fn is_not_correct(diff: i32, direction: bool) -> bool {
    diff.abs() < 1 || diff.abs() > 3 || (diff > 0) != direction || diff == 0
}
