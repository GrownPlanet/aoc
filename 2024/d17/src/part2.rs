use aoc_helper::read_file;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let (_, prog) = input.split_once("\n\n").unwrap();
    let instr: Vec<usize> = prog
        .split_once(" ")
        .unwrap()
        .1
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    println!("{:?}", find(&instr, 0));
}

fn find(instr: &[usize], ans: usize) -> Option<usize> {
    if instr.len() == 0 {
        return Some(ans);
    }

    let s = if instr.len() == 16 {
       1
    } else {
       0
    };
    for t in s..8 {
        let a = (ans << 3) + t;
        let mut b = a & 0b111;
        b = b ^ 3;
        let c = a >> b;
        b = b ^ c;
        b = b ^ 3;

        if b & 0b111 == instr[instr.len() - 1] {
            let sub = find(&instr[..instr.len() - 1], a);
            if sub == None {
                continue;
            } else {
                return sub;
            }
        }
    }

    None
}
