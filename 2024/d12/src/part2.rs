use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let mut map: Vec<Vec<char>> = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let mut char_ar_par = vec![];

    let mut visited = HashSet::new();
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if !visited.contains(&(x, y)) {
                let mut to_append = HashSet::new();
                let (ar, par) = find_area_and_par(&mut map, (x, y), &mut to_append, &mut HashMap::new());
                for a in to_append.iter() {
                    visited.insert(*a);
                }
                char_ar_par.push((ar, par));
            }
        }
    }

    let result: usize = char_ar_par
        .iter()
        .map(|(p, a)| p * a)
        .sum();

    println!("{}", result);
}

fn find_area_and_par(
    map: &mut Vec<Vec<char>>,
    (x, y): (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    walled: &mut HashMap<(usize, usize), [bool; 4]>,
) -> (usize, usize) {
    let char = map[y][x];
    visited.insert((x, y));

    let mut area = 1;
    let mut par = 0;

    let lims = (map.len() - 1, map[0].len() - 1);

    for dir in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        if let Some((dx, dy)) = get_offset_position((x, y), dir, lims) {
            if map[dy][dx] == char && !visited.contains(&(dx, dy)) {
                let (aa, ap) = find_area_and_par(map, (dx, dy), visited, walled);
                area += aa;
                par += ap;
            } else if !visited.contains(&(dx, dy)) {
                if count_wall(map, (x, y), visited, walled, dir) {
                    par += 1;
                }
            }
        } else {
            if count_wall(map, (x, y), visited, walled, dir) {
                par += 1;
            }
        }
    }

    (area, par)
}

fn count_wall(
    map: &mut Vec<Vec<char>>,
    (x, y): (usize, usize),
    _visited: &mut HashSet<(usize, usize)>,
    walled: &mut HashMap<(usize, usize), [bool; 4]>,
    dir: (i32, i32),
) -> bool {
    let char = map[y][x];
    let lims = (map.len() - 1, map[0].len() - 1);
    let wall_index = dir_to_index(dir);
    if let Some(walls) = walled.get(&(x, y)) {
        if walls[wall_index] {
            return false;
        }
    }

    let mut w = [false; 4];
    w[wall_index] = true;

    for per_dir in per_dirs(dir) {
        let mut nx = x;
        let mut ny = y;
        'w: while let Some((ox, oy)) = get_offset_position((nx, ny), per_dir, lims) {
            let mut continue_s = map[oy][ox] == char;
            if let Some((looks_x, looks_y)) = get_offset_position((ox, oy), dir, lims) {
                continue_s = continue_s && (map[looks_y][looks_x] != char);
            }
            if continue_s {
                walled
                    .entry((ox, oy))
                    .and_modify(|e| e[wall_index] = true)
                    .or_insert(w);
            } else {
                break 'w;
            }
            nx = ox;
            ny = oy
        }
    }

    walled
        .entry((x, y))
        .and_modify(|e| e[wall_index] = true)
        .or_insert(w);

    true
}

fn dir_to_index(dir: (i32, i32)) -> usize {
    match dir {
        (0, -1) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (1, 0) => 3,
        _ => panic!("invalid dir!"),
    }
}

fn per_dirs(dir: (i32, i32)) -> [(i32, i32); 2] {
    match dir {
        (-1, 0) | (1, 0) => [(0, 1), (0, -1)],
        (0, -1) | (0, 1) => [(1, 0), (-1, 0)],
        _ => panic!("invalid dir!"),
    }
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
