use aoc_helper::{read_file, to_map};

const LEN: usize = 5;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");
    
    let mut locks: Vec<[u8; LEN]> = vec![];
    let mut keys: Vec<[u8; LEN]> = vec![];
    
    for heightmap in input.split("\n\n") {
        let map = to_map(heightmap);
        let start_char = map[0][0];

        let mut hm = [0; LEN];

        for x in 0..LEN {
            let mut y = 1;
            while map[y][x] == start_char {
                hm[x] += 1;
                y += 1;
            }
        }

        match start_char {
            '#' => locks.push(hm),
            '.' => keys.push(hm),
            _ => panic!()
        }
    }

    let mut result = 0;
    for lock in &locks {
        for key in &keys {
            if fits(*lock, *key) { result += 1; }
        }
    }

    println!("{}", result);
}

fn fits(lock: [u8; 5], key: [u8;  5]) -> bool {
    for i in 0..LEN {
        if lock[i] > key[i] {
            return false;
        }
    }

    return true;
}
