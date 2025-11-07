use std::{
    collections::{HashMap, VecDeque},
    usize,
};

use aoc_helper::{calculate_limits, get_offset_position, read_file, to_map};

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

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

    let result_1 = shortest_path(start, 1, end, &map);
    let result_2 = shortest_path(start, 0, end, &map);

    let result = [result_1, result_2.map(|x| x + 1000)]
        .into_iter()
        .flatten()
        .min()
        .unwrap();

    println!("{}", result);
}

fn shortest_path(
    pos: (usize, usize),
    dir: u8,
    end: (usize, usize),
    map: &Vec<Vec<char>>,
) -> Option<usize> {
    let lims = calculate_limits(map);

    let mut queue: VecDeque<((usize, usize), u8, usize)> = VecDeque::new();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

    visited.insert(pos, 0);
    queue.push_back((pos, dir, 0));

    let mut solutions = vec![];

    while !queue.is_empty() {
        let (curpos, curdir, len) = queue.pop_front().unwrap();

        if curpos == end {
            solutions.push(len);
            continue;
        }
        if map[curpos.1][curpos.0] == '#' {
            continue;
        }

        let updated_pos = get_offset_position(curpos, dir_to_offset(curdir), lims).unwrap();
        match visited.get(&updated_pos) {
            None => {
                queue.push_back((updated_pos, curdir, len + 1));
                visited.insert(updated_pos, len + 1);
            }
            Some(&l) => {
                if len + 1 < l {
                    queue.push_back((updated_pos, curdir, len + 1));
                    visited.insert(updated_pos, len + 1);
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
                queue.push_back((updated_pos_l, left_turn, len + 1002));
                visited.insert(updated_pos_l, len + 1002);
            }
            Some(&l) => {
                if len + 1002 < l {
                    queue.push_back((updated_pos_l, left_turn, len + 1002));
                    visited.insert(updated_pos_l, len + 1002);
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
                queue.push_back((updated_pos_r, right_turn, len + 1002));
                visited.insert(updated_pos_r, len + 1002);
            }
            Some(&l) => {
                if len + 1002 < l {
                    queue.push_back((updated_pos_r, right_turn, len + 1002));
                    visited.insert(updated_pos_r, len + 1002);
                }
            }
        }
    }

    solutions.iter().min().copied()
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
