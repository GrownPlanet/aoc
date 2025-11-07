use std::collections::HashMap;

use aoc_helper::read_file;

/*
* mistakes:
* - z09 ~ hnd
* - z16 ~ tdv
* - z23 ~ bks
* - tjp ~ nrn
*/

pub fn solve() {
    // let input = read_file("test_input.txt");
    let input = read_file("input.txt");

    let mut combinations = HashMap::new();
    input.split_once("\n\n").unwrap().1.lines().for_each(|l| {
        let parts: Vec<&str> = l.split_whitespace().collect();
        // ZZZ = XXX op YYY
        [parts[4], parts[0], parts[1], parts[2]];
        combinations.insert(parts[4], [parts[0], parts[1], parts[2]]);
    });

    let mut mistakes = vec!["z09", "hnd", "z16", "tdv", "z23", "bks", "tjp", "nrn"];
    mistakes.sort();
    for mistake in mistakes {
        print!("{},", mistake);
    }
    println!();
    
    // print_combs(&combinations, "z03", 0);
    // for n in 0..45 {
    //     let zstr: &str = &format!("z{:02}", n);
    //     let vz = verify_z(zstr, n, &combinations);
    //     println!("{}", vz);
    //     if !vz {
    //         return;
    //     }
    // }
}

fn verify_z(wire: &str, num: usize, comb: &HashMap<&str, [&str; 3]>) -> bool {
    println!("vz: {}, {}", wire, num);
    let form = comb.get(&wire).unwrap();

    if form[1] != "XOR" {
        return false;
    }

    if num == 0 {
        if (form[0] == "x00" && form[2] == "y00") || (form[2] == "x00" && form[0] == "y00") {
            return true;
        }
        return false;
    }

    return (verify_interm_xor(form[0], num, comb) && verify_car_bit(form[2], num, comb))
        || (verify_interm_xor(form[2], num, comb) && verify_car_bit(form[0], num, comb));
}

fn verify_interm_xor(wire: &str, num: usize, comb: &HashMap<&str, [&str; 3]>) -> bool {
    println!("vx: {}, {}", wire, num);
    let form = comb.get(&wire).unwrap();
    if form[1] != "XOR" {
        return false;
    }

    let xstr: &str = &format!("x{:02}", num);
    let ystr: &str = &format!("y{:02}", num);
    return (form[0] == xstr && form[2] == ystr) || (form[2] == xstr && form[0] == ystr);
}

fn verify_car_bit(wire: &str, num: usize, comb: &HashMap<&str, [&str; 3]>) -> bool {
    println!("vc: {}, {}", wire, num);
    let form = comb.get(&wire).unwrap();

    if num == 1 {
        return ((form[0] == "x00" && form[2] == "y00") || (form[2] == "x00" && form[0] == "y00"))
            && form[1] == "AND";
    }
    if form[1] != "OR" {
        return false;
    }

    return (verify_dir_car(form[0], num - 1, comb) && verify_re_car(form[2], num - 1, comb))
        || (verify_dir_car(form[2], num - 1, comb) && verify_re_car(form[0], num - 1, comb));
}

fn verify_dir_car(wire: &str, num: usize, comb: &HashMap<&str, [&str; 3]>) -> bool {
    println!("vd: {}, {}", wire, num);
    let form = comb.get(&wire).unwrap();

    if form[1] != "AND" {
        return false;
    }

    let xstr: &str = &format!("x{:02}", num);
    let ystr: &str = &format!("y{:02}", num);
    return (form[0] == xstr && form[2] == ystr) || (form[2] == xstr && form[0] == ystr);
}

fn verify_re_car(wire: &str, num: usize, comb: &HashMap<&str, [&str; 3]>) -> bool {
    println!("vr: {}, {}", wire, num);
    let form = comb.get(&wire).unwrap();

    if form[1] != "AND" {
        return false;
    }

    return (verify_interm_xor(form[0], num, comb) && verify_car_bit(form[2], num, comb))
        || (verify_interm_xor(form[2], num, comb) && verify_car_bit(form[0], num, comb));
}

fn print_combs(combinations: &HashMap<&str, [&str; 3]>, str: &str, ind: usize) {
    let combof = combinations.get(str).unwrap();
    println!("{}{} ({})", " ".repeat(ind), combof[1], str);

    if combof[0].starts_with("x") || combof[0].starts_with("y") {
        println!("{}{}", " ".repeat(ind + 2), combof[0])
    } else {
        print_combs(combinations, combof[0], ind + 2);
    }

    if combof[2].starts_with("x") || combof[2].starts_with("y") {
        println!("{}{}", " ".repeat(ind + 2), combof[2])
    } else {
        print_combs(combinations, combof[2], ind + 2);
    }
}
