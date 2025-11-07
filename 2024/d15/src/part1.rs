use aoc_helper::{calculate_limits, get_offset_position, read_file, to_map};

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut map = to_map(map);
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

        if let Some((mut x, mut y)) = find_dot(&map, pos, dir) {
            let it = match mv {
                '>' | '<' => pos.0.min(x)..=pos.0.max(x),
                '^' | 'v' => pos.1.min(y)..=pos.1.max(y),
                _ => panic!(),
            };

            for _ in it {
                let (nx, ny) = get_offset_position((x, y), (-dir.0, -dir.1), lims).unwrap();
                map[y][x] = map[ny][nx];
                (x, y) = (nx, ny);
            }

            map[pos.1][pos.0] = '.';
            pos = get_offset_position(pos, dir, lims).unwrap();
            map[pos.1][pos.0] = '@';
        }
    }

    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                result += y * 100 + x;
            }
        }
    }
    println!("{}", result);
}

fn find_dot(map: &[Vec<char>], mut pos: (usize, usize), dir: (i32, i32)) -> Option<(usize, usize)> {
    let lims = calculate_limits(map);
    while let Some((x, y)) = get_offset_position(pos, dir, lims) {
        match map[y][x] {
            '.' => return Some((x, y)),
            '#' => return None,
            _ => pos = (x, y),
        }
    }
    return None;
}
