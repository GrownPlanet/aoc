use aoc_helper::read_file;
use std::collections::HashMap;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let (initials, code) = input.split_once("\n\n").unwrap();
    let mut wire_values: HashMap<&str, u8> = HashMap::new();

    for init in initials.lines() {
        let (code, value) = init.split_once(": ").unwrap();
        let value = value.parse::<u8>().unwrap();
        wire_values.insert(code, value);
    }

    let code: Vec<&str> = code.lines().collect();
    let mut waiting_indices: Vec<usize> = vec![];
    let mut i = 0;

    loop {
        match try_exec(code[i], &wire_values) {
            Some((v, k)) => {
                wire_values.insert(k, v);
                iter_waiting_i(&mut waiting_indices, &code, &mut wire_values);
            }
            None => waiting_indices.push(i),
        }

        i += 1;
        if i >= code.len() {
            break;
        }
    }

    let mut i = 0;
    let mut result = 0;
    while let Some(outval) = wire_values.get(format!("z{:02}", i).as_str()) {
        result += (*outval as usize) << i;
        i += 1;
    }

    println!("{:#b}", result);
    println!("{}", result);
}

fn iter_waiting_i<'a>(
    waiting_indices: &mut Vec<usize>,
    code: &Vec<&'a str>,
    wire_values: &mut HashMap<&'a str, u8>,
) {
    let mut changed = false;

    *waiting_indices = waiting_indices
        .iter()
        .filter(|&i| match try_exec(code[*i], wire_values) {
            Some((v, k)) => {
                wire_values.insert(k, v);
                changed = true;
                false
            }
            None => true,
        })
        .cloned()
        .collect::<Vec<usize>>();

    if changed {
        iter_waiting_i(waiting_indices, code, wire_values);
    }
}

fn try_exec<'a>(line: &'a str, wire_values: &HashMap<&'a str, u8>) -> Option<(u8, &'a str)> {
    let instructions: Vec<&str> = line.split_whitespace().collect();

    let n1 = wire_values.get(instructions[0])?;
    let n2 = wire_values.get(instructions[2])?;

    Some((
        match instructions[1] {
            "AND" => n1 & n2,
            "XOR" => n1 ^ n2,
            "OR" => n1 | n2,
            _ => panic!(),
        },
        instructions[4],
    ))
}
