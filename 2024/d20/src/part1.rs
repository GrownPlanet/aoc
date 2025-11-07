use std::{collections::{HashSet, VecDeque}, usize};

use aoc_helper::{calculate_limits, get_offset_position, read_file, to_map};

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let map = to_map(&input);

    let mut startp = (0, 0);
    let mut endp = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                startp = (x, y);
            } else if map[y][x] == 'E' {
                endp = (x, y);
            }
        }
    }

    let base_solve = base_solve(startp, endp, &map).unwrap();
    println!("{}", base_solve);

    let cheat_solves = cheat_solve(startp, endp, &map);
    let result = cheat_solves
        .iter()
        .filter(|&&s| base_solve as i32 - s as i32 >= 100)
        .count();
    println!("{}", result);
}

fn base_solve(startp: (usize, usize), endp: (usize, usize), map: &Vec<Vec<char>>) -> Option<usize> {
    let lims = calculate_limits(map);

    let mut shortest_path: Option<usize> = None;
    
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((startp, 0));
    visited.insert(startp);

    while !queue.is_empty() {
        let (pos, len) = queue.pop_front().unwrap();
        if pos == endp {
            shortest_path = Some(len);
            break;
        }

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

    shortest_path
}

fn cheat_solve(startp: (usize, usize), endp: (usize, usize), map: &Vec<Vec<char>>) -> Vec<usize> {
    let lims = calculate_limits(map);

    let mut paths = vec![];
    
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((startp, 0));
    visited.insert(startp);

    while !queue.is_empty() {
        let (pos, len) = queue.pop_front().unwrap();

        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some((x, y)) = get_offset_position(pos, offset, lims) {
                if visited.contains(&(x, y)) {
                    continue;
                }

                if map[y][x] == '#' {
                    if let Some((x2, y2)) = get_offset_position((x, y), offset, lims) {
                        if map[y2][x2] == '#' {
                            continue;
                        }
                        if let Some(sol) = base_solve((x2, y2), endp, map) {
                            paths.push(sol + len + 2);
                        }
                    }
                    continue;
                }
                queue.push_back(((x, y), len + 1));
                visited.insert((x, y));
            }
        }
    }

    paths
}
