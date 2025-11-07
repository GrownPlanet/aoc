use std::fs;

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();
    let mut result = 0;
    for line in input.lines() {
        let (total, numbers) = line.trim().split_once(": ").unwrap();
        let total = total.parse::<i64>().unwrap();
        let numbers: Vec<i64> = 
            numbers.split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect();

        if sums_up(total, numbers) {
            result += total;
        }
    }
    println!("{}", result);
}

fn sums_up(total: i64, numbers: Vec<i64>) -> bool {
    if numbers.len() == 1 {
        if total == numbers[0] {
            return true;
        } else {
            return false;
        }
    } 

    let mut nums_plus = numbers.clone();
    nums_plus[0] = nums_plus[0] + nums_plus[1];
    nums_plus.remove(1);

    let mut nums_times = numbers.clone();
    nums_times[0] = nums_times[0] * nums_times[1];
    nums_times.remove(1);

    sums_up(total, nums_plus) || sums_up(total, nums_times)
}
