use std::collections::HashMap;

use aoc_helper::{calculate_limits, get_offset_position, read_file};

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let numpad = vec![
        vec!['7', '8', '9'],
        vec!['4', '5', '6'],
        vec!['1', '2', '3'],
        vec!['#', '0', 'A'],
    ];
    let npsp = find_pos(&numpad, 'A').unwrap();

    let controlpad = vec![
        vec!['#', '^', 'A'],
        vec!['<', 'v', '>']
    ];
    let cpsp = find_pos(&controlpad, 'A').unwrap();

    let mut startposs = vec![npsp];
    for _ in 0..25 {
        startposs.push(cpsp);
    }

    let mut result = 0;

    let mut memo: HashMap<((usize, usize), (usize, usize), usize), usize> = HashMap::new();
    for string in input.lines() {
        let mut res = 0;
        for ch in string.chars() {
            let (np, len) = shortest_sequence_it1(ch, startposs.clone(), &numpad, &controlpad, &mut memo);
            startposs[0] = np;
            res += len;
        }
        let nump = string[0..3].parse::<usize>().unwrap();
        result += res * nump;
        println!("{:?} done!", string);
    }

    println!("{}", result);
}

fn shortest_sequence_it1(
    input: char,
    mut curposs: Vec<(usize, usize)>,
    numpad: &Vec<Vec<char>>,
    controlpad: &Vec<Vec<char>>,
    memo: &mut HashMap<((usize, usize), (usize, usize), usize), usize>,
) -> ((usize, usize), usize) {
    let (x1, y1) = curposs[0];
    let (x2, y2) = find_pos(numpad, input).unwrap();

    let options: Vec<String> = get_paths((x1, y1), (x2, y2), numpad);

    let res = options
        .iter()
        .map(|op|
            op
                .chars()
                .map(|c| {
                    let new_curposs = curposs.clone()[1..].to_vec();
                    let (np, len) = shortest_sequence_it2(c, new_curposs, controlpad, memo);
                    curposs[1] = np;
                    len
                })
                .sum::<usize>()
        )
        .min()
        .unwrap();

    return ((x2, y2), res);
}

fn shortest_sequence_it2(
    input: char,
    mut curposs: Vec<(usize, usize)>,
    controlpad: &Vec<Vec<char>>,
    memo: &mut HashMap<((usize, usize), (usize, usize), usize), usize>,
) -> ((usize, usize), usize) {
    if curposs.len() == 1 {
        return shortest_sequence_it3(input, curposs[0], controlpad);
    }

    let (x1, y1) = curposs[0];
    let (x2, y2) = find_pos(controlpad, input).unwrap();

    if let Some(ret) = memo.get(&((x1, y1), (x2, y2), curposs.len())) {
        return ((x2, y2), *ret);
    }

    let options: Vec<String> = get_paths((x1, y1), (x2, y2), controlpad);

    let res = options
        .iter()
        .map(|op|
            op
                .chars()
                .map(|c| {
                    let new_curposs = curposs.clone()[1..].to_vec();
                    let (np, len) = shortest_sequence_it2(c, new_curposs, controlpad, memo);
                    curposs[1] = np;
                    len
                })
                .sum::<usize>()
        )
        .min()
        .unwrap();

    memo.insert(((x1, y1), (x2, y2), curposs.len()), res);

    return ((x2, y2), res);
}

fn shortest_sequence_it3(
    input: char,
    curposs: (usize, usize),
    controlpad: &Vec<Vec<char>>,
) -> ((usize, usize), usize) {
    let (x1, y1) = curposs;
    let (x2, y2) = find_pos(controlpad, input).unwrap();

    let options: Vec<String> = get_paths((x1, y1), (x2, y2), controlpad);

    let res = options.iter().map(|s| s.len()).min().unwrap();

    return ((x2, y2), res);
}

fn get_paths((x1, y1): (usize, usize), (x2, y2): (usize, usize), map: &Vec<Vec<char>>) -> Vec<String> {
    let mut options = vec![];

    let lims = calculate_limits(map);

    let dx = x2 as i32 - x1 as i32;
    let dy = y2 as i32 - y1 as i32;

    let xc = if dx > 0 { '>' } else { '<' };
    let yc = if dy > 0 { 'v' } else { '^' };

    let (mut x, mut y) = (x1, y1);
    let mut op1 = vec![];
    for _ in 0..dx.abs() {
        op1.push(xc);
    }
    for _ in 0..dy.abs() {
        op1.push(yc);
    }
    let mut pases_through_empty1 = false;
    for d in &op1 {
        let off = match d {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!(),
        };
        (x, y) = get_offset_position((x, y), off, lims).unwrap();
        if map[y][x] == '#' {
            pases_through_empty1 = true;
            break;
        }
    }
    op1.push('A');

    (x, y) = (x1, y1);
    let mut op2 = vec![];
    for _ in 0..dy.abs() {
        op2.push(yc);
    }
    for _ in 0..dx.abs() {
        op2.push(xc);
    }
    let mut pases_through_empty2 = false;
    for d in &op2 {
        let off = match d {
            '^' => (0, -1),
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            _ => panic!(),
        };
        (x, y) = get_offset_position((x, y), off, lims).unwrap();
        if map[y][x] == '#' {
            pases_through_empty2 = true;
            break;
        }
    }
    op2.push('A');

    if !pases_through_empty2 && op1 != op2 {
        options.push(op2);
    }
    if !pases_through_empty1 {
        options.push(op1);
    }

    options
        .iter()
        .map(|v| v.iter().collect::<String>())
        .collect()
}


fn find_pos(map: &Vec<Vec<char>>, ch: char) -> Option<(usize, usize)> {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == ch {
                return Some((x, y));
            }
        }
    }
    return None;
}
