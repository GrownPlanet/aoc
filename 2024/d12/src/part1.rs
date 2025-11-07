use std::{collections::HashSet, fs};

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut char_ar_par: Vec<(usize, usize)> = vec![];

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x].is_alphabetic() {
                let (ar, par) = find_area_and_par(&mut map, (x, y), &mut HashSet::new());
                char_ar_par.push((ar, par));
            }
        }
    }

    let result: usize = char_ar_par.iter().map(|(p, a)| p * a).sum();

    println!("{}", result);
}

// -> (area, parimeter)
fn find_area_and_par(
    map: &mut Vec<Vec<char>>,
    (x, y): (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let char = map[y][x];
    visited.insert((x, y));
    map[y][x] = '.';

    let mut area = 1;
    let mut par = 0;

    let lims = (map.len() - 1, map[0].len() - 1);

    for offset in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        if let Some((dx, dy)) = get_offset_position((x, y), offset, lims) {
            if map[dy][dx] == char {
                let (aa, ap) = find_area_and_par(map, (dx, dy), visited);
                area += aa;
                par += ap;
            } else if !visited.contains(&(dx, dy)) {
                par += 1;
            }
        } else {
            par += 1;
        }
    }

    (area, par)
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
