use std::collections::{HashMap, HashSet};

use aoc_helper::read_file;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let connections_raw: Vec<(&str, &str)> =
        input.lines().map(|l| l.split_once('-').unwrap()).collect();

    let mut connections: HashMap<&str, Vec<&str>> = HashMap::new();
    for (c1, c2) in connections_raw {
        connections
            .entry(c1)
            .and_modify(|v| v.push(c2))
            .or_insert(vec![c2]);
        connections
            .entry(c2)
            .and_modify(|v| v.push(c1))
            .or_insert(vec![c1]);
    }

    let mut triangles: HashSet<[&str; 3]> = HashSet::new();
    for (computer, cons) in &connections {
        for con in cons {
            for concon in connections.get(con).unwrap() {
                if connections.get(concon).unwrap().contains(computer) {
                    let mut triag = [*computer, *con, *concon];
                    triag.sort();
                    triangles.insert(triag);
                }
            }
        }
    }

    let mut count = 0;
    for triag in triangles {
        for comp in triag {
            if comp.starts_with('t') {
                count += 1;
                break;
            }
        }
    }

    println!("{}", count);
}
