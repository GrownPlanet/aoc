use std::{collections::HashSet, fs};

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let lims = (map[0].len() - 1, map.len() - 1);

    let mut spos = (0, 0);
    'f: for (y, v1) in map.iter().enumerate() {
        for (x, v2) in v1.iter().enumerate() {
            if *v2 == '^' {
                spos = (x, y);
                break 'f;
            }
        }
    }
    let mut gpos = spos;
    map[gpos.1][gpos.0] = '.';


    let mut dir = 0; // 0 => up; 1 => right; 2 => down; 3 => left
    let dir_move = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut passed_positions = HashSet::new();
    passed_positions.insert(gpos);

    let mut result = 0;
    let mut leftmap = false;
    while !leftmap {
        let mv = dir_move[dir];
        if let Some((x, y)) = get_offset_position(gpos, mv, lims) {
            if map[y][x] == '#' {
                dir = (dir + 1) % 4;
            } else {
                map[y][x] = '#';
                if !passed_positions.contains(&(x, y)) && check_loop(spos, &map) {
                    result += 1;
                }
                map[y][x] = '.';

                gpos = (x, y);
                passed_positions.insert(gpos);
            }
        } else {
            leftmap = true;
        }
    }

    println!("{}", result);
}

fn check_loop(
    mut gpos: (usize, usize),
    map: &Vec<Vec<char>>,
) -> bool {
    let mut dir = 0;
    let dir_move = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let lims = (map[0].len() - 1, map.len() - 1);

    let mut passed_positions = HashSet::new();
    passed_positions.insert((gpos, dir));

    let mut leftmap = false;
    while !leftmap {
        let mv = dir_move[dir];
        if let Some((x, y)) = get_offset_position(gpos, mv, lims) {
            if map[y][x] == '#' {
                dir = (dir + 1) % 4;
                passed_positions.insert((gpos, dir));
            } else {
                gpos = (x, y);
                if passed_positions.contains(&(gpos, dir)) {
                    return true;
                }
                passed_positions.insert((gpos, dir));
            }
        } else {
            leftmap = true;
        }
    }

    false
}

fn get_offset_position(
    (x, y): (usize, usize),
    (dx, dy): (i32, i32),
    (xlim, ylim): (usize, usize),
) -> Option<(usize, usize)> {
    let x_with_offset = x as i32 + dx;
    let y_with_offset = y as i32 + dy;

    if x_with_offset < 0
        || y_with_offset < 0
        || x_with_offset > xlim as i32
        || y_with_offset > ylim as i32
    {
        return None;
    }

    Some((x_with_offset as usize, y_with_offset as usize))
}
