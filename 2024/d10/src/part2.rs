use std::{fs, usize};

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|l| l
            .chars()
            .map(|c| (c as u8 - '0' as u8) as usize)
            .collect::<Vec<usize>>()
        )
        .collect();

    let result = map
        .iter()
        .enumerate()
        .map(|(y, v)| {
            v
                .iter()
                .enumerate()
                .map(|(x, &p)| if p == 0 { 
                    count_hiking_trails((x, y), &map)
                } else { 
                    0 
                })
                .sum::<usize>()
        })
        .sum::<usize>();
    println!("{}", result);
}

fn count_hiking_trails((x, y): (usize, usize), map: &[Vec<usize>]) -> usize {
    let cur_num = map[y][x];
    if cur_num == 9 {
        return 1;
    }
    let lims = (map.len() - 1, map[0].len() - 1);
    let mut next_pos = vec![];
    for dir in [
        (-1, 0),
        (0, -1),
        (0, 1),
        (1, 0),
    ] {
        if let Some((ox, oy)) = get_offset_position((x, y), dir, lims) {
            if map[oy][ox] == cur_num + 1 {
                next_pos.push((ox, oy));
            }
        }
    }

    let hiking_trails: usize = next_pos
        .iter()
        .map(|&p| count_hiking_trails(p, map))
        .sum();

    hiking_trails
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

