use std::fs;

pub fn solve() {
    let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    // let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let mut id = 0;
    let mut disk: Vec<Sector> = input
        .chars()
        .enumerate()
        .flat_map(|(i, ch)| {
            if i % 2 == 0 {
                id += 1;
                vec![Sector::File(id - 1); (ch as u8 - '0' as u8) as usize]
            } else {
                vec![Sector::Space; (ch as u8 - '0' as u8) as usize]
            }
        })
        .collect();

    'f: for i in (0..disk.len()).rev() {
        let sector = disk[i];

        match sector {
            Sector::Space => (),
            Sector::File(_) => {
                let fes = first_empty_sector(&disk);
                disk.swap(i, fes);

                if is_defracted(&disk) {
                    break 'f;
                }
            },
        }
    }

    let result = disk
        .iter()
        .enumerate()
        .map(|(i, &v)| match v {
            Sector::Space => 0,
            Sector::File(m) => i * m,
        })
        .sum::<usize>();

    println!("{}", result);
}

fn first_empty_sector(disk: &[Sector]) -> usize {
    disk
        .iter()
        .position(|&s| s == Sector::Space)
        .expect("no empty sectors")
}

fn is_defracted(disk: &[Sector]) -> bool {
    let mut passed_space = false;
    for i in disk {
        match *i {
            Sector::Space => passed_space = true,
            Sector::File(_) => if passed_space { return false },
        }
    }

    return true;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sector {
    File(usize),
    Space
}
