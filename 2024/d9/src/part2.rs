use std::fs;

pub fn solve() {
    // let input = fs::read_to_string("test_input.txt").unwrap().trim().to_string();
    let input = fs::read_to_string("input.txt").unwrap().trim().to_string();

    let mut id = 0;
    let mut disk: Vec<Sector> = input
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            if i % 2 == 0 {
                id += 1;
                Sector::File(((ch as u8 - '0' as u8) as usize, id - 1))
            } else {
                Sector::Space((ch as u8 - '0' as u8) as usize)
            }
        })
        .collect();

    for i in (0..disk.len()).rev() {
        let sector = disk[i];

        match sector {
            Sector::Space(_) => (),
            Sector::File((len, _)) => {
                let mut fes = None;
                'f: for k in 0..i {
                    let sector = disk[k];
                    match sector {
                        Sector::File(_) => (),
                        Sector::Space(space_len) => {
                            if space_len >= len {
                                fes = Some((k, space_len));
                                break 'f;
                            }
                        }
                    }
                }

                if let Some((k, space_len)) = fes {
                    disk[i] = Sector::Space(len);
                    disk[k] = Sector::Space(space_len - len);
                    disk.insert(k, sector);
                }
            },
        }
    }

    let mut result = 0;
    let mut mult = 0;
    for i in disk {
        match i {
            Sector::Space(len) => mult += len,
            Sector::File((len, id)) => {
                for _ in 0..len {
                    result += id * mult;
                    mult += 1;
                }
            }
        }
    }
    println!("{}", result);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sector {
    // len, id
    File((usize, usize)),
    // len
    Space(usize)
}
