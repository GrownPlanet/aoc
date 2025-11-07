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
    let mut startposs = [npsp, cpsp, cpsp];

    let mut result = 0;

    for string in input.lines() {
        let mut res = String::new();
        for ch in string.chars() {
            let (np, s) = shortest_sequence_it1(ch, startposs, &numpad, &controlpad);
            startposs[0] = np;
            res.push_str(&s);
        }
        let nump = string[0..3].parse::<usize>().unwrap();
        result += res.len() * nump;
    }

    println!("{}", result);
}

fn shortest_sequence_it1(
    input: char,
    mut curposs: [(usize, usize); 3],
    numpad: &Vec<Vec<char>>,
    controlpad: &Vec<Vec<char>>
) -> ((usize, usize), String) {
    let (x1, y1) = curposs[0];
    let (x2, y2) = find_pos(numpad, input).unwrap();

    let options: Vec<String> = get_paths((x1, y1), (x2, y2), numpad);

    let res = options
        .iter()
        .map(|op|
            op
                .chars()
                .flat_map(|c| {
                    let (np, s) = shortest_sequence_it2(c, [curposs[1], curposs[2]], controlpad);
                    curposs[1] = np;
                    s.chars().collect::<Vec<char>>()
                })
                .collect::<String>()
        )
        .min_by_key(|s| s.len())
        .unwrap()
        .to_string();

    return ((x2, y2), res);
}

fn shortest_sequence_it2(
    input: char,
    mut curposs: [(usize, usize); 2],
    controlpad: &Vec<Vec<char>>,
) -> ((usize, usize), String) {
    let (x1, y1) = curposs[0];
    let (x2, y2) = find_pos(controlpad, input).unwrap();

    let options: Vec<String> = get_paths((x1, y1), (x2, y2), controlpad);

    let res = options
        .iter()
        .map(|op|
            op
                .chars()
                .flat_map(|c| {
                    let (np, s) = shortest_sequence_it3(c, curposs[1], controlpad);
                    curposs[1] = np;
                    s.chars().collect::<Vec<char>>()
                })
                .collect::<String>()
        )
        .min_by_key(|s| s.len())
        .unwrap()
        .to_string();

    return ((x2, y2), res);
}

fn shortest_sequence_it3(
    input: char,
    curposs: (usize, usize),
    controlpad: &Vec<Vec<char>>,
) -> ((usize, usize), String) {
    let (x1, y1) = curposs;
    let (x2, y2) = find_pos(controlpad, input).unwrap();

    let options: Vec<String> = get_paths((x1, y1), (x2, y2), controlpad);

    let res = options.iter().min_by_key(|s| s.len()).unwrap();

    return ((x2, y2), res.to_string());
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
