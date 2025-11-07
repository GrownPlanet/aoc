use std::collections::{HashSet, VecDeque};

use aoc_helper::{get_offset_position, read_file};

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    // parsing input
    let map_size = (70, 70);
    // let map_size = (6, 6);

    let mut map: Vec<Vec<char>> = vec![vec!['.'; map_size.0 + 1]; map_size.1 + 1];

    for (i, location) in input.lines().enumerate() {
        if i >= 1024 {
        // if i >= 12 {
            break;
        }

        let (xs, ys) = location.split_once(',').unwrap();
        let x = xs.parse::<usize>().unwrap();
        let y = ys.parse::<usize>().unwrap();
        map[y][x] = '#';
    }

    let startp = (0, 0);
    let endp = (map_size.0, map_size.1);

    // bfs
    let mut shortest_path: Option<usize> = None;
    
    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((startp, 0));
    visited.insert(startp);

    while !queue.is_empty() {
        let (pos, len) = queue.pop_front().unwrap();
        if pos == endp {
            shortest_path = Some(len);
            break;
        }

        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some((x, y)) = get_offset_position(pos, offset, map_size) {
                if map[y][x] == '#' || visited.contains(&(x, y)) {
                    continue;
                }
                queue.push_back(((x, y), len + 1));
                visited.insert((x, y));
            }
        }
    }

    println!("{:?}", shortest_path);
}
