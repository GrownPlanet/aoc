use std::{collections::HashMap, fs};

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();
    let lims = (map.len() - 1, map[0].len() - 1);

    let mut letter_locs: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (y, col) in map.iter().enumerate() {
        for (x, ch) in col.iter().enumerate() {
            if *ch != '.' {
                letter_locs.entry(*ch).or_insert_with(Vec::new).push((x, y));
            }
        }
    }

    let mut result = 0;
    for (key, vec) in letter_locs.iter() {
        for l1 in vec {
            for l2 in vec {
                if let Some((x, y)) = get_antinode(*l1, *l2, lims) {
                    if map[y][x] != *key && map[y][x] != '#' {
                        result += 1;
                        map[y][x] = '#';
                    }
                }
            }
        }
    }
    println!("{}", result);
}

fn get_antinode(
    l1: (usize, usize), l2: (usize, usize), lims: (usize, usize)
) -> Option<(usize, usize)> {
    let dir = (
        l1.0 as i32 - l2.0 as i32,
        l1.1 as i32 - l2.1 as i32,
    );
    get_offset_position(l1, dir, lims)
}

fn get_offset_position(
    (x, y): (usize, usize), (dx, dy): (i32, i32), (xlim, ylim): (usize, usize)
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
