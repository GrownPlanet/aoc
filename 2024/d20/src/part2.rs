use std::{collections::{HashSet, VecDeque}, usize};

use aoc_helper::{calculate_limits, get_offset_position, read_file, to_map};

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let map = to_map(&input);

    let mut startp = (0, 0);
    'f: for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                startp = (x, y);
                break 'f;
            }        
        }
    }

    let base_solve = base_solve(startp, &map);
    let cheat_solves = cheat_solve(startp, &base_solve)
        .iter()
        .filter(|&&v| v >= 100)
        .count();

    println!("{:?}", cheat_solves);
}

fn base_solve(startp: (usize, usize), map: &Vec<Vec<char>>) -> Vec<Vec<i32>> {
    let lims = calculate_limits(map);

    let mut return_map = vec![vec![-1; map[0].len()]; map.len()];

    let mut queue: VecDeque<((usize, usize), i32)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((startp, 0));
    visited.insert(startp);

    while !queue.is_empty() {
        let (pos, len) = queue.pop_front().unwrap();
        return_map[pos.1][pos.0] = len;

        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some((x, y)) = get_offset_position(pos, offset, lims) {
                if map[y][x] == '#' || visited.contains(&(x, y)) {
                    continue;
                }
                queue.push_back(((x, y), len + 1));
                visited.insert((x, y));
            }
        }
    }

    return return_map;
}

fn cheat_solve(startp: (usize, usize), map: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut saved_times = vec![];

    let lims = calculate_limits(map);

    let mut queue: VecDeque<((usize, usize), i32)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((startp, 0));
    visited.insert(startp);

    while !queue.is_empty() {
        let ((x, y), len) = queue.pop_front().unwrap();

        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some((x2, y2)) = get_offset_position((x, y), offset, lims) {
                if map[y2][x2] == -1 || visited.contains(&(x2, y2)) {
                    continue;
                }

                let cel_rad = cel_rad_20((x, y), lims);
                for ((x2, y2), len) in cel_rad {
                    let num = map[y2][x2];
                    if num == -1 { continue; }
                    let st = num - (map[y][x] + len);
                    saved_times.push(st);
                }

                queue.push_back(((x2, y2), len + 1));
                visited.insert((x2, y2));
            }
        }
    }

    saved_times
}

fn cel_rad_20(startp: (usize, usize), lims: (usize, usize)) -> Vec<((usize, usize), i32)> {
    let mut res = vec![];

    let mut queue: VecDeque<((usize, usize), i32)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((startp, 0));
    visited.insert(startp);

    while !queue.is_empty() {
        let (pos, len) = queue.pop_front().unwrap();
        let nl = len + 1;

        if nl > 20 {
            continue;
        }

        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some((x, y)) = get_offset_position(pos, offset, lims) {
                if visited.contains(&(x, y)) {
                    continue;
                }
                queue.push_back(((x, y), nl));
                visited.insert((x, y));
                res.push(((x, y), nl));
            }
        }
    }

    res
}
