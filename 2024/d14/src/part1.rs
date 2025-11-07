use aoc_helper::read_file;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");
    let mut robots: Vec<(i32, i32, i32, i32)> = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(' ').unwrap();
            let (_, p) = p.split_at(2);
            let (_, v) = v.split_at(2);
            let (px, py) = p.split_once(',').unwrap();
            let (vx, vy) = v.split_once(',').unwrap();

            return (
                px.parse::<i32>().unwrap(),
                py.parse::<i32>().unwrap(),
                vx.parse::<i32>().unwrap(),
                vy.parse::<i32>().unwrap()
            );
        })
        .collect();

    // let map_size = (11, 7);
    let map_size = (101, 103);

    for _ in 0..100 {
        robots = robots
            .iter()
            .map(|&(px, py, vx, vy)| {
                let mut npx = (px + vx) % map_size.0;
                let mut npy = (py + vy) % map_size.1;

                if npx < 0 { npx += map_size.0 }
                if npy < 0 { npy += map_size.1 }

                (npx, npy, vx, vy)
            })
            .collect();
    }

    let mut quadrants = [0; 4];

    for (x, y, _, _) in robots {
        if let Some(q) = determine_quadrant((x, y), map_size) {
            quadrants[q] += 1;
        }   
    }

    let result = quadrants.iter().product::<usize>();

    println!("{}", result);
}

fn determine_quadrant((x, y): (i32, i32), (map_w, map_h): (i32, i32)) -> Option<usize> {
    if x == (map_w - 1) / 2 ||
        y == (map_h - 1) / 2 {
        return None;
    }

    let right_q = x < (map_w / 2);
    let upper_q = y < (map_h / 2);

    return Some(match (right_q, upper_q) {
        (true, true) => 0,
        (false, true) => 1,
        (true, false) => 2,
        (false, false) => 3,
    });
}
