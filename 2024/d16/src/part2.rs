use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use aoc_helper::{calculate_limits, get_offset_position, read_file, to_map};

pub fn solve() {
    let input = read_file("test_input.txt");
    // let input = read_file("input.txt");

    let map = to_map(&input);
    let mut start = (0, 0);
    let mut end = (0, 0);
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'S' {
                start = (x, y);
            } else if map[y][x] == 'E' {
                end = (x, y);
            }
        }
    }

    let result = shortest_path(start, end, &map);

    println!("{}", result);
}

fn shortest_path(pos: (usize, usize), end: (usize, usize), map: &Vec<Vec<char>>) -> usize {
    let lims = calculate_limits(map);

    let mut queue: VecDeque<((usize, usize), u8, usize, Vec<(usize, usize)>)> = VecDeque::new();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

    visited.insert(pos, 0);
    queue.push_back((pos, 1, 0, vec![]));

    visited.insert(pos, 0);
    queue.push_back((pos, 0, 1000, vec![]));

    let mut solutions = vec![];

    while !queue.is_empty() {
        let (curpos, curdir, len, path) = queue.pop_front().unwrap();

        if curpos == end {
            solutions.push((len, path));
            continue;
        }
        if map[curpos.1][curpos.0] == '#' {
            continue;
        }

        let updated_pos = get_offset_position(curpos, dir_to_offset(curdir), lims).unwrap();
        match visited.get(&updated_pos) {
            None => {
                push_path(
                    vec![updated_pos],
                    path.clone(),
                    curdir,
                    len + 1,
                    &mut queue,
                    &mut visited,
                );
            }
            Some(&l) => {
                if len + 1 <= l {
                    push_path(
                        vec![updated_pos],
                        path.clone(),
                        curdir,
                        len + 1,
                        &mut queue,
                        &mut visited,
                    );
                }
            }
        }

        if map[updated_pos.1][updated_pos.0] == '#' || updated_pos == end {
            continue;
        }

        let left_turn = (curdir + 1) % 4;
        let updated_pos_l =
            get_offset_position(updated_pos, dir_to_offset(left_turn), lims).unwrap();
        match visited.get(&updated_pos_l) {
            None => {
                push_path(
                    vec![updated_pos, updated_pos_l],
                    path.clone(),
                    left_turn,
                    len + 1002,
                    &mut queue,
                    &mut visited,
                );
            }
            Some(&l) => {
                if len + 1002 <= l {
                    push_path(
                        vec![updated_pos, updated_pos_l],
                        path.clone(),
                        left_turn,
                        len + 1002,
                        &mut queue,
                        &mut visited,
                    );
                }
            }
        }

        let right_turn = if curdir as i32 - 1 >= 0 {
            curdir - 1
        } else {
            3
        };
        let updated_pos_r =
            get_offset_position(updated_pos, dir_to_offset(right_turn), lims).unwrap();
        match visited.get(&updated_pos_r) {
            None => {
                push_path(
                    vec![updated_pos, updated_pos_r],
                    path.clone(),
                    right_turn,
                    len + 1002,
                    &mut queue,
                    &mut visited,
                );
            }
            Some(&l) => {
                if len + 1002 <= l {
                    push_path(
                        vec![updated_pos, updated_pos_r],
                        path.clone(),
                        right_turn,
                        len + 1002,
                        &mut queue,
                        &mut visited,
                    );
                }
            }
        }
    }

    if solutions.is_empty() {
        return 0;
    }

    let mut min_len = solutions[0].0;
    let mut paths = vec![];

    for (len, path) in solutions {
        if len > min_len {
            continue;
        }
        if len < min_len {
            min_len = len;
            paths = vec![];
        }
        paths.push(path);
    }

    let mut best_locations: Vec<(usize, usize)> = paths.iter().flatten().copied().collect();
    best_locations.sort();
    best_locations.dedup();

    best_locations.len() + 1
}

fn dir_to_offset(dir: u8) -> (i32, i32) {
    match dir {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => panic!(),
    }
}

fn push_path(
    poss: Vec<(usize, usize)>,
    mut path: Vec<(usize, usize)>,
    dir: u8,
    new_len: usize,
    queue: &mut VecDeque<((usize, usize), u8, usize, Vec<(usize, usize)>)>,
    visited: &mut HashMap<(usize, usize), usize>,
) {
    for pos in &poss {
        path.push(*pos);
    }
    queue.push_back((poss[poss.len() - 1], dir, new_len, path));
    visited.insert(poss[poss.len() - 1], new_len);
}
