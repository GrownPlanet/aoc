use std::collections::{HashMap, HashSet};

use aoc_helper::read_file;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let mut nums = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut sequence_count: HashMap<[i64; 4], i64> = HashMap::new();

    for num in nums.iter_mut() {
        let mut sequence = [0; 4];
        let mut seen_seq: HashSet<[i64; 4]> = HashSet::new();

        let mut prev = *num % 10;
        *num = next(*num);
        let mut curr = *num % 10;
        sequence[0] = curr - prev;
        for j in 1..4 {
            *num = next(*num);
            prev = curr;
            curr = *num % 10;
            sequence[j] = curr - prev;
        }
        sequence_count.entry(sequence).and_modify(|c| *c += curr).or_insert(curr);
        seen_seq.insert(sequence);

        for _ in 0..(2000 - 4) {
            *num = next(*num);
            prev = curr;
            curr = *num % 10;

            shift_add(&mut sequence, curr - prev);

            if !seen_seq.contains(&sequence) {
                sequence_count.entry(sequence).and_modify(|c| *c += curr).or_insert(curr);
                seen_seq.insert(sequence);
            }
        }
    }

    let mut max_sequence = [0; 4];
    let mut max_num = 0;

    for (seq, num) in sequence_count.iter() {
        if *num > max_num {
            max_sequence = *seq;
            max_num = *num;
        }
    }


    println!("{:?}: {}", max_sequence, max_num);
}

fn shift_add(seq: &mut [i64; 4], elem: i64) {
    for i in 0..3 {
        seq[i] = seq[i + 1];
    }
    seq[3] = elem;
}

fn next(mut num: i64) -> i64 {
    fn prune(n1: i64) -> i64 { n1 % 16777216 }

    num = prune((num * 64) ^ num);
    num = prune((num / 32) ^ num);
    num = prune((num * 2048) ^ num);

    num
}
