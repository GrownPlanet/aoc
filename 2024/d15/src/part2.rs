use aoc_helper::{calculate_limits, get_offset_position, read_file, to_map};

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut map = to_map(map)
        .iter()
        .map(|r| {
            r.iter()
                .flat_map(|&c| match c {
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    _ => [c; 2],
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>();
    let lims = calculate_limits(&map);

    let mut pos = (0, 0);
    'f: for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '@' {
                pos = (x, y);
                break 'f;
            }
        }
    }

    for mv in moves.chars() {
        if mv == '\n' {
            continue;
        }

        let dir = match mv {
            '>' => (1, 0),
            '<' => (-1, 0),
            'v' => (0, 1),
            '^' => (0, -1),
            _ => panic!(),
        };

        if move_possible(pos, 1, dir, &map) {
            move_boxes(pos, 1, dir, &mut map);
            pos = get_offset_position(pos, dir, lims).unwrap();
        }
    }

    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == '[' {
                result += y * 100 + x;
            }
        }
    }
    println!("{}", result);
}

fn move_possible(
    (x, y): (usize, usize),
    width: usize,
    dir: (i32, i32),
    map: &Vec<Vec<char>>,
) -> bool {
    if map[y][x] == '.' {
        return true;
    } else if map[y][x] == '#' {
        return false;
    }

    let lims = calculate_limits(&map);

    let mut result = true;
    match dir {
        (0, 1) | (0, -1) => {
            for xo in 0..width {
                let check_pos = get_offset_position((x + xo, y), dir, lims).unwrap();
                let check_ch = map[check_pos.1][check_pos.0];

                match check_ch {
                    '[' => result = result && move_possible(check_pos, 2, dir, map),
                    ']' => {
                        result = result
                            && move_possible(
                                get_offset_position(check_pos, (-1, 0), lims).unwrap(),
                                2,
                                dir,
                                map,
                            )
                    }
                    '#' => result = false,
                    _ => (),
                }
            }
        }
        (1, 0) | (-1, 0) => {
            let check_x = if dir == (1, 0) {
                (x as i32 + (width as i32 * dir.0)) as usize
            } else {
                (x as i32 + (width as i32 * dir.0)) as usize + (width - 1)
            };
            let check_pos = (check_x, y);
            let check_ch = map[check_pos.1][check_pos.0];

            match check_ch {
                '[' => result = result && move_possible(check_pos, 2, dir, map),
                ']' => {
                    result = result
                        && move_possible(
                            get_offset_position(check_pos, (-1, 0), lims).unwrap(),
                            2,
                            dir,
                            map,
                        )
                }
                '#' => result = false,
                _ => (),
            }
        }
        _ => (),
    }

    result
}

fn move_boxes((x, y): (usize, usize), width: usize, dir: (i32, i32), map: &mut Vec<Vec<char>>) {
    if map[y][x] == '.' || map[y][x] == '#' {
        return;
    }

    let lims = calculate_limits(&map);

    match dir {
        (0, 1) | (0, -1) => {
            for xo in 0..width {
                let check_pos = get_offset_position((x + xo, y), dir, lims).unwrap();
                let check_ch = map[check_pos.1][check_pos.0];

                match check_ch {
                    '[' => move_boxes(check_pos, 2, dir, map),
                    ']' => {
                        if xo == 1 {
                            break;
                        }
                        move_boxes(
                            get_offset_position(check_pos, (-1, 0), lims).unwrap(),
                            2,
                            dir,
                            map,
                        )
                    }
                    _ => (),
                }
            }
        }
        (1, 0) | (-1, 0) => {
            let check_x = if dir == (1, 0) {
                (x as i32 + (width as i32 * dir.0)) as usize
            } else {
                (x as i32 + (width as i32 * dir.0)) as usize + (width - 1)
            };
            let check_pos = (check_x, y);
            let check_ch = map[check_pos.1][check_pos.0];

            match check_ch {
                '[' => move_boxes(check_pos, 2, dir, map),
                ']' => move_boxes(
                    get_offset_position(check_pos, (-1, 0), lims).unwrap(),
                    2,
                    dir,
                    map,
                ),
                _ => (),
            }
        }
        _ => (),
    }

    if width == 1 {
        let mv_pos = get_offset_position((x, y), dir, lims).unwrap();
        map[mv_pos.1][mv_pos.0] = map[y][x];
        map[y][x] = '.';
    } else if width == 2 {
        let mv_pos_1 = get_offset_position((x, y), dir, lims).unwrap();
        let mv_pos_2 = get_offset_position((x + 1, y), dir, lims).unwrap();

        let ch_1 = map[y][x];
        let ch_2 = map[y][x + 1];

        map[mv_pos_1.1][mv_pos_1.0] = ch_1;
        map[mv_pos_2.1][mv_pos_2.0] = ch_2;

        if dir != (-1, 0) {
            map[y][x] = '.';
        }
        if dir != (1, 0) {
            map[y][x + 1] = '.';
        }
    }
}
