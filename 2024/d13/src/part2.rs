use aoc_helper::read_file;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let machines: Vec<&str> = input.split("\n\n").collect();

    let mut result = 0;
    for machine in machines {
        let lines = machine.lines().collect::<Vec<&str>>();

        let xa = to_num(12, 13, lines[0]);
        let ya = to_num(18, 19, lines[0]);

        let xb = to_num(12, 13, lines[1]);
        let yb = to_num(18, 19, lines[1]);

        let (_, prize) = lines[2].split_once(' ').unwrap();
        let (xprize, yprize) = prize.split_once(", ").unwrap();
        let xprize = xprize.split_once('=').unwrap().1.parse::<i64>().unwrap() + 10000000000000;
        let yprize = yprize.split_once('=').unwrap().1.parse::<i64>().unwrap() + 10000000000000;

        let (am, bm) = find_comb(
            xa as f64,
            xb as f64,
            xprize as f64,
            ya as f64,
            yb as f64,
            yprize as f64,
        );
        if xa * am + xb * bm == xprize && ya * am + yb * bm == yprize {
            result += am * 3 + bm;
        }
    }

    println!("{}", result);
}

fn to_num(from: usize, to: usize, string: &str) -> i64 {
    std::str::from_utf8(&string.as_bytes()[from..=to])
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

fn find_comb(a1: f64, b1: f64, c1: f64, a2: f64, b2: f64, c2: f64) -> (i64, i64) {
    let y = (a1 * c2 - a2 * c1) / (a1 * b2 - a2 * b1);
    let x = (c1 / a1) - (b1 / a1) * y;
    (x.round() as i64, y.round() as i64)
}
