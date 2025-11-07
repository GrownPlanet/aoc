use std::fs;

pub fn solve() {
    // let raw_input = fs::read_to_string("test_input.txt").unwrap();
    let raw_input = fs::read_to_string("input.txt").unwrap();
    let input = raw_input.trim();

    let mut result = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        'f: for i in 0..nums.len() {
            let mut cl_nums = nums.clone();
            cl_nums.remove(i);

            for i in 0..cl_nums.len() - 1 {
                let diff = cl_nums[i] - cl_nums[i + 1];
                if is_not_correct(diff, cl_nums[0] - cl_nums[1] > 0) {
                    continue 'f;
                }
            }

            println!("{:?}; {}", nums, nums[i]);
            result += 1;
            break;
        }

    }

    println!("{}", result);
}

fn is_not_correct(diff: i32, direction: bool) -> bool {
    diff.abs() < 1 || diff.abs() > 3 || (diff > 0) != direction || diff == 0
}
