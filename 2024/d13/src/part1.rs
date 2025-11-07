use aoc_helper::read_file;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let machines: Vec<&str> = input.split("\n\n").collect();

    let mut result = 0;
    for machine in machines {
        let mut min_comb: Option<i32> = None;

        let lines = machine.lines().collect::<Vec<&str>>();

        let xa = to_num(12, 13, lines[0]);
        let ya = to_num(18, 19, lines[0]);

        let xb = to_num(12, 13, lines[1]);
        let yb = to_num(18, 19, lines[1]);

        let (_, prize) = lines[2].split_once(' ').unwrap();
        let (xprize, yprize) = prize.split_once(", ").unwrap();
        let xprize = xprize.split_once('=').unwrap().1.parse::<i32>().unwrap();
        let yprize = yprize.split_once('=').unwrap().1.parse::<i32>().unwrap();

        for a_mult in 0..100 {
            for b_mult in 0..100 {
                if xa * a_mult + xb * b_mult == xprize && ya * a_mult + yb * b_mult == yprize {
                    min_comb = match min_comb {
                        Some(n) => Some((a_mult * 3 + b_mult).min(n)),
                        None => Some(a_mult * 3 + b_mult),
                    }
                }
            }
        }

        match min_comb {
            Some(n) => result += n,
            None => (),
        }
    }

    println!("{}", result);
}

fn to_num(from: usize, to: usize, string: &str) -> i32 {
    std::str::from_utf8(&string.as_bytes()[from..=to])
        .unwrap()
        .parse::<i32>()
        .unwrap()
}
