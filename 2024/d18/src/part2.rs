use std::collections::{HashSet, VecDeque};

use aoc_helper::{calculate_limits, get_offset_position, read_file};

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let locations: Vec<(usize, usize)> = input
        .lines()
        .map(|l| {
            let (xs, ys) = l.split_once(',').unwrap();
            let x = xs.parse::<usize>().unwrap();
            let y = ys.parse::<usize>().unwrap();
            (x, y)
        })
        .collect();

    let map_size = (70, 70);
    // let map_size = (6, 6);

    let startp = (0, 0);
    let endp = (map_size.0, map_size.1);

    let mut lo = 0;
    let mut hi = locations.len() - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        let mut map: Vec<Vec<char>> = vec![vec!['.'; map_size.0 + 1]; map_size.1 + 1];

        for i in 0..mid {
            let (x, y) = locations[i];
            map[y][x] = '#';
        }

        if go_to_end(startp, endp, &map) {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }

    println!("{:?}", locations[hi]);
}

fn go_to_end(startp: (usize, usize), endp: (usize, usize), map: &Vec<Vec<char>>) -> bool {
    let lims = calculate_limits(map);

    let mut queue: VecDeque<((usize, usize), usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((startp, 0));
    visited.insert(startp);

    while !queue.is_empty() {
        let (pos, len) = queue.pop_front().unwrap();
        if pos == endp {
            return true;
        }

        for offset in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if let Some((x, y)) = get_offset_position(pos, offset, lims) {
                if map[y][x] == '#' || visited.contains(&(x, y)) {
                    continue;
                }
                queue.push_back(((x, y), len + 1));
                visited.insert((x, y));
            }
        }
    }

    return false;
}
