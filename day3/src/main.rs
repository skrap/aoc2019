fn main() {
    run_part_1(include_str!("input.txt"));
    run_part_2(include_str!("input.txt"));
}

#[derive(Debug)]
struct Seg {
    dir: char,
    dist: isize,
}

fn run_part_1(input: &str) {
    let directions: Vec<Vec<Seg>> = input
        .lines()
        .map(|line| {
            line.trim()
                .split(',')
                .map(|s| {
                    let (dir, dist) = s.split_at(1);
                    Seg {
                        dir: dir.chars().next().unwrap(),
                        dist: dist.parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect();

    let mut board = std::collections::HashMap::new();
    for (wire_num, wire) in directions.iter().enumerate() {
        let mut pos = [0_isize, 0_isize];
        for seg in wire.iter() {
            for _step in 0..seg.dist {
                match seg.dir {
                    'R' => pos[0] += 1,
                    'L' => pos[0] -= 1,
                    'U' => pos[1] -= 1,
                    'D' => pos[1] += 1,
                    _ => panic!("unknown dir {}", seg.dir),
                }
                board.entry(pos).or_insert([false, false])[wire_num] = true;
            }
        }
    }

    let mut crosses: Vec<_> = board
        .iter()
        .filter_map(|(pos, v)| if *v == [true, true] { Some(pos) } else { None })
        .collect();

    fn mhd(pos: &[isize; 2]) -> isize {
        pos[0].abs() + pos[1].abs()
    }

    crosses.sort_by(|a, b| mhd(a).cmp(&mhd(b)));
    dbg!(crosses[0], mhd(crosses[0]));
}

fn run_part_2(input: &str) {
    let directions: Vec<Vec<Seg>> = input
        .lines()
        .map(|line| {
            line.trim()
                .split(',')
                .map(|s| {
                    let (dir, dist) = s.split_at(1);
                    Seg {
                        dir: dir.chars().next().unwrap(),
                        dist: dist.parse().unwrap(),
                    }
                })
                .collect()
        })
        .collect();

    let mut board = std::collections::HashMap::new();
    for (wire_num, wire) in directions.iter().enumerate() {
        let mut pos = [0_isize, 0_isize];
        let mut steps = 0;
        for seg in wire.iter() {
            for _step in 0..seg.dist {
                match seg.dir {
                    'R' => pos[0] += 1,
                    'L' => pos[0] -= 1,
                    'U' => pos[1] -= 1,
                    'D' => pos[1] += 1,
                    _ => panic!("unknown dir {}", seg.dir),
                }
                steps += 1;
                board.entry(pos).or_insert([None, None])[wire_num].get_or_insert(steps);
            }
        }
    }

    let mut crosses: Vec<_> = board
        .iter()
        .filter_map(|(_pos, v)| {
            if let [Some(v0), Some(v1)] = v {
                Some(v0 + v1)
            } else {
                None
            }
        })
        .collect();

    crosses.sort();
    dbg!(crosses[0]);
}
