use std::fs;

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let lims = (map[0].len() - 1, map.len() - 1);

    let mut gpos = (0, 0);
    'f: for (y, v1) in map.iter().enumerate() {
        for (x, v2) in v1.iter().enumerate() {
            if *v2 == '^' {
                gpos = (x, y);
                break 'f;
            }
        }
    }
    map[gpos.1][gpos.0] = 'X';

    let mut dir = 0; // 0 => up; 1 => right; 2 => down; 3 => left
    let dir_move = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut leftmap = false;
    while !leftmap {
        let mv = dir_move[dir];
        if let Some((x, y)) = get_offset_position(gpos, mv, lims) {
            if map[y][x] == '#' {
                dir = (dir + 1) % 4;
            } else {
                gpos = (x, y);
                map[y][x] = 'X';
            }
        } else {
            leftmap = true;
        }
    }

    let result: i32 = map
        .iter()
        .map(|r| r.iter().fold(0, |acc, v| acc + (*v == 'X') as u8 as i32))
        .sum();

    println!("{}", result);
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
