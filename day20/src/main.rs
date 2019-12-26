use std::collections::{HashMap, VecDeque};

fn main() {
    let input = include_bytes!("input.txt");
    let map = Map::new(input);
    println!(
        "Part 1 best to ZZ: {}",
        do_part1(map.clone(), map.start_pos, map.end_pos).unwrap().0
    );
    println!(
        "Part 2 example best to ZZ: {}",
        do_part2(Map::new(include_bytes!("part2_example.txt")))
    );
    println!("Part 2 best to ZZ: {}", do_part2(map.clone()));
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
            if (5..(self.width - 5)).contains(&pos.0) && (5..(self.height - 5)).contains(&pos.1) {
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

fn do_part1(map: Map, start: Pos, end: Pos) -> Option<(usize, isize)> {
    do_reachables(&map, start, true).get(&end).copied()
}

fn do_reachables(map: &Map, start: Pos, portals: bool) -> HashMap<Pos, (usize, isize)> {
    let mut best = HashMap::new();
    let mut tasks = VecDeque::new();
    tasks.push_back((start, 0, 0));

    while let Some((pos, traveled, level)) = tasks.pop_back() {
        if let Some((before, _level)) = best.get(&pos) {
            if *before <= traveled {
                continue;
            }
        }
        best.insert(pos, (traveled, level));

        for step in &[pos.down(), pos.left(), pos.right(), pos.up()] {
            if map.at(pos) == b'.' {
                tasks.push_back((*step, traveled + 1, level));
            }
        }
        if portals {
            if let Some((step, levelmod)) = map.use_portal(pos) {
                tasks.push_back((step, traveled + 1, level + levelmod));
            }
        }
    }

    best
}

fn do_part2(map: Map) -> usize {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    let mut reachables: HashMap<Pos, _> = HashMap::new();

    let mut tasks = BinaryHeap::new();
    tasks.push(Reverse((0, 0, map.start_pos)));

    let mut best = HashMap::new();
    let mut best_finish = None;

    while let Some(Reverse((dist, level, pos))) = tasks.pop() {
        if let Some(prev_best) = best_finish {
            if prev_best <= dist {
                continue; // stop searching if we've solved it in fewer steps.
            }
        }

        if let Some(&best_dist) = best.get(&(pos, level)) {
            if best_dist <= dist {
                continue;
            }
        }
        // println!(
        //     "new best to {} level {}: {}",
        //     std::str::from_utf8(&map.portal_name_at(pos).unwrap()).unwrap(),
        //     level,
        //     dist
        // );

        best.insert((pos, level), dist);
        if pos == map.end_pos && level == 0 {
            println!("new best finish: {}", dist);
            best_finish = Some(dist);
        }

        let entry = reachables.entry(pos).or_insert_with(|| {
            // find the traveled dist from start_pos to all portals
            let mut result: Vec<(Pos, usize, isize)> = Vec::new();
            let from_start = do_reachables(&map, pos, false);
            for (&end_pos, &(traveled, shouldbezero)) in from_start.iter() {
                assert_eq!(shouldbezero, 0);
                if pos == end_pos {
                    continue;
                }
                if map.portal_name_at(end_pos).is_some() {
                    result.push((end_pos, traveled, 0));
                    if let Some((portaled_pos, levelmod)) = map.use_portal(end_pos) {
                        result.push((portaled_pos, traveled + 1, levelmod));
                    }
                }
            }
            result
        });

        for &(newpos, dist_mod, level_mod) in entry.iter() {
            if level + level_mod >= 0 && dist_mod > 0 {
                tasks.push(Reverse((dist + dist_mod, level + level_mod, newpos)));
            }
        }
    }

    *best.get(&(map.end_pos, 0)).unwrap()
}
