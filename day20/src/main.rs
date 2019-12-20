use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_bytes!("input.txt");
    let map = Map::new(input);
    do_part1(map.clone());
    do_part2(map.clone());
}

type Pos = (isize, isize);

#[derive(Clone)]
struct Map {
    backing: Vec<u8>,
    width: isize,
    height: isize,
    portals: HashMap<Pos, Pos>,
    start_pos: Pos,
    end_pos: Pos,
}

type Portal = [u8; 2];

use itertools::Itertools;

impl Map {
    fn new(input: &[u8]) -> Map {
        let width = (input.iter().position(|&b| b == b'\n').unwrap() + 1) as isize;
        let height = input.len() as isize / width;
        assert_eq!(width * height, input.len() as isize);
        let mut map = Map {
            backing: Vec::from(input),
            width,
            height,
            portals: HashMap::new(),
            start_pos: (0, 0),
            end_pos: (0, 0),
        };
        let mut portals: HashMap<Portal, Pos> = HashMap::new();
        for pos in (0..width).cartesian_product(0..height) {
            if let Some(name) = map.portal_name_at(pos) {
                if &name == b"AA" {
                    map.start_pos = pos;
                } else if &name == b"ZZ" {
                    map.end_pos = pos;
                } else if let Some(otherpos) = portals.remove(&name) {
                    map.portals.insert(pos, otherpos);
                    map.portals.insert(otherpos, pos);
                } else {
                    portals.insert(name, pos);
                }
            }
        }
        map
    }

    fn at(&self, pos: Pos) -> u8 {
        self.backing[(pos.1 * self.width + pos.0) as usize]
    }

    fn use_portal(&self, pos: Pos) -> Option<(Pos, isize)> {
        let levelmod =
            if (5..(self.width - 5)).contains(&pos.0) || (5..(self.height - 5)).contains(&pos.1) {
                1
            } else {
                -1
            };
        self.portals.get(&pos).map(|p| (*p, levelmod))
    }

    fn portal_name_at(&self, pos: Pos) -> Option<[u8; 2]> {
        if self.at(pos) == b'.' {
            let pairs = [
                pos.up().up(),
                pos.up(),
                pos.down(),
                pos.down().down(),
                pos.left().left(),
                pos.left(),
                pos.right(),
                pos.right().right(),
            ];
            for p in pairs.chunks_exact(2) {
                let (at1, at2) = (self.at(p[0]), self.at(p[1]));
                if at1.is_ascii_alphabetic() && at2.is_ascii_alphabetic() {
                    return Some([at1, at2]);
                }
            }
        }
        None
    }
}

trait PosStuff {
    fn up(&self) -> Self;
    fn down(&self) -> Self;
    fn left(&self) -> Self;
    fn right(&self) -> Self;
}

impl PosStuff for Pos {
    fn up(&self) -> Self {
        (self.0, self.1 - 1)
    }
    fn down(&self) -> Self {
        (self.0, self.1 + 1)
    }
    fn left(&self) -> Self {
        (self.0 - 1, self.1)
    }
    fn right(&self) -> Self {
        (self.0 + 1, self.1)
    }
}

fn do_part1(map: Map) {
    let mut best = HashMap::new();
    let mut tasks = VecDeque::new();
    tasks.push_back((map.start_pos, 0));

    while let Some((pos, traveled)) = tasks.pop_back() {
        if let Some(before) = best.get(&pos) {
            if *before <= traveled {
                continue;
            }
        }
        best.insert(pos, traveled);

        for step in &[pos.down(), pos.left(), pos.right(), pos.up()] {
            if map.at(pos) == b'.' {
                tasks.push_back((*step, traveled + 1));
            }
        }
        if let Some((step, _levelmod)) = map.use_portal(pos) {
            tasks.push_back((step, traveled + 1));
        }
    }

    println!("Part 1 best to ZZ: {}", best.get(&map.end_pos).unwrap());
}

fn do_part2(map: Map) -> bool {
    let mut best = HashMap::new();
    let mut tasks = VecDeque::new();
    tasks.push_back((map.start_pos, 0, 0, HashMap::new()));

    while let Some((pos, level, traveled, mut gates)) = tasks.pop_back() {
        if let Some(before) = best.get(&(pos, level)) {
            if *before <= traveled {
                continue;
            }
        }
        best.insert((pos, level), traveled);

        for step in &[pos.down(), pos.left(), pos.right(), pos.up()] {
            if map.at(pos) == b'.' {
                tasks.push_back((*step, level, traveled + 1, gates.clone()));
            }
        }
        if let Some((step, levelmod)) = map.use_portal(pos) {
            let gate_times = gates.entry(pos).or_insert(0);
            *gate_times += 1;
            tasks.push_back((step, level + levelmod, traveled + 1, gates));
        }
    }
    if let Some(best) = best.get(&(map.end_pos, 0)) {
        println!(
            "Part 2 best to ZZ: {}",
            best
        );
        true
    } else {
        println!("no route to ZZ");
        false
    }
}
