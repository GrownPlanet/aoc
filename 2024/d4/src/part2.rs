use std::{fs, usize};

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let map = input
        .lines()
        .map(|c| c.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut result = 0;
    for y in 1..map.len() - 1 {
        'f: for x in 1..map[y].len() - 1 {
            if map[y][x] != 'A' {
                continue 'f;
            }

            let offsets = [
                ((-1, -1), (1, 1)),
                ((-1, 1), (1, -1)),
            ];

            for (o1, o2) in offsets {
                let xo1 = (x as i32 + o1.0) as usize;
                let yo1 = (y as i32 + o1.1) as usize;
                let xo2 = (x as i32 + o2.0) as usize;
                let yo2 = (y as i32 + o2.1) as usize;

                if !(
                    (map[yo1][xo1] == 'M' && map[yo2][xo2] == 'S')
                    || (map[yo1][xo1] == 'S' && map[yo2][xo2] == 'M')
                ) {
                    continue 'f;
                }
            }

            result += 1;
        }
    }

    println!("{}", result);
}
