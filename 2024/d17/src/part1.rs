use aoc_helper::read_file;

const A_REG: usize = 0;
const B_REG: usize = 1;
const C_REG: usize = 2;

const ADV: usize = 0;
const BXL: usize = 1;
const BST: usize = 2;
const JNZ: usize = 3;
const BXC: usize = 4;
const OUT: usize = 5;
const BDV: usize = 6;
const CDV: usize = 7;

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let (reg, prog) = input.split_once("\n\n").unwrap();

    let mut registers = [0; 3];
    let mut rp = 0;
    for r in reg.lines() {
        let val = r.split_once(": ").unwrap().1.parse::<usize>().unwrap();
        registers[rp] = val;
        rp += 1;
    }

    let instructions = prog
        .split_once(" ")
        .unwrap()
        .1
        .split(',')
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    interpret(registers, instructions);
}

fn interpret(mut registers: [usize; 3], instructions: Vec<usize>) {
    let mut inst_p = 0;

    while inst_p < instructions.len() - 1 {
        let instruction = instructions[inst_p];
        let operand = instructions[inst_p + 1];

        match instruction {
            ADV => { // 0
                let comb = get_comb(operand, registers);
                registers[A_REG] = (registers[A_REG] as f64 / (2_f64.powi(comb as i32))) as usize;
            }
            BXL => { // 1
                registers[B_REG] = registers[B_REG] ^ operand;
            }
            BST => { // 2
                let comb = get_comb(operand, registers);
                registers[B_REG] = comb % 8;
            }
            JNZ => { // 3
                if registers[A_REG] == 0 {
                    inst_p += 1;
                } else {
                    inst_p = operand;
                }
            }
            BXC => { // 4
                registers[B_REG] = registers[B_REG] ^ registers[C_REG];
            }
            OUT => { // 5
                let comb = get_comb(operand, registers);
                print!("{},", comb % 8);
            }
            BDV => { // 6
                let comb = get_comb(operand, registers);
                registers[B_REG] = (registers[A_REG] as f64 / (2_f64.powi(comb as i32))) as usize;
            }
            CDV => { // 7
                let comb = get_comb(operand, registers);
                registers[C_REG] = (registers[A_REG] as f64 / (2_f64.powi(comb as i32))) as usize;
            }
            _ => panic!("Unknown instruction: {}", instruction),
        }

        if instruction != JNZ {
            inst_p += 2;
        }
    }

    println!();
}

fn get_comb(val: usize, registers: [usize; 3]) -> usize {
    if val <= 3 {
        val
    } else if val >= 4 && val <= 6 {
        registers[val - 4]
    } else {
        panic!("Invalid combo value: {}", val)
    }
}

/*
* 0 -> adv -> combo   -> A / (2^op) -> A
* 1 -> bxl -> literal -> XOR B op   -> B
* 2 -> bst -> combo   -> op % 8     -> B
* 3 -> jnz -> literal -> A == 0 ? {} : { inst_p = op } -> N/A
* 4 -> bxc -> N/A     -> XOR B C    -> B
* 5 -> out -> combo   -> op % 8     -> print
* 6 -> bdv -> combo   -> A / (2^op) -> B
* 7 -> cdv -> combo   -> A / (2^op) -> C
*
*
* combo:
* 0..3 -> 0..3
* 4    -> A
* 5    -> B
* 6    -> C
* 7    -> invalid prog
*/
