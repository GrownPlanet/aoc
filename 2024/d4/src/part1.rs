use std::{fs, usize};

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let map = input
        .lines()
        .map(|c| c.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut result = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            result += count_word("XMAS".as_bytes(), (x, y), &map);
        }
    }

    println!("{}", result);
}

fn count_word(word: &[u8], (x, y): (usize, usize), map: &Vec<Vec<char>>) -> i32 {
    fn is_word_dir(
        word: &[u8], (x, y): (usize, usize), d: (i32, i32), map: &Vec<Vec<char>>
    ) -> bool {
        if word.is_empty() {
            true
        } else if map[y][x] != word[0] as char {
            false
        } else {
            let next_word = &word[1..word.len()];
            let lims = (map[0].len() - 1, map.len() - 1);

            if let Some(np) = get_offset_position((x, y), d, lims) {
                is_word_dir(next_word, np, d, map)
            } else if next_word.is_empty() {
                true
            } else {
                false
            }
        }
    }

    if map[y][x] != word[0] as char {
        return 0;
    }

    let next_word = &word[1..word.len()];
    let lims = (map[0].len() - 1, map.len() - 1);
    get_offset_positions((x, y), lims)
        .iter()
        .map(|(pos, dir)| is_word_dir(next_word, *pos, *dir, map))
        .fold(0, |acc, b| acc + b as u8 as i32)
}

fn get_offset_positions(
    pos: (usize, usize),
    lims: (usize, usize),
) -> Vec<((usize, usize), (i32, i32))> {
    let mut offset_positions = vec![];
    let offsets = [(0, 1), (0, -1), (1, 1), (1, -1), (1, 0), (-1, 1), (-1, -1), (-1, 0)];
    for d in offsets {
        if let Some(op) = get_offset_position(pos, d, lims) {
            offset_positions.push((op, d));
        }
    }
    offset_positions
}

fn get_offset_position((x, y): (usize, usize), (dx, dy): (i32, i32), (xlim, ylim): (usize, usize)) -> Option<(usize, usize)> {
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
