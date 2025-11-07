use aoc_helper::read_file;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let mut nums = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for num in nums.iter_mut() {
        for _ in 0..2000 {
            *num = next(*num);
        }
    }

    let sum = nums.iter().sum::<i64>();
    println!("{}", sum); }

fn next(mut num: i64) -> i64 {
    fn prune(n1: i64) -> i64 {
        n1 % 16777216
    }

    num = prune((num * 64) ^ num);
    num = prune((num / 32) ^ num);
    num = prune((num * 2048) ^ num);

    num
}
