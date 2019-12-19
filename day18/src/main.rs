use std::collections::{HashMap, HashSet, VecDeque};

type Pos = (isize, isize);

struct Map {
    backing: Vec<Tile>,
    width: usize,
}

impl Map {
    fn get(&self, pos: &Pos) -> Option<&Tile> {
        if pos.0 < 0 || pos.1 < 0 {
            None
        } else {
            self.backing
                .get(pos.1 as usize * self.width + pos.0 as usize)
        }
    }
}

fn main() {
    do_part1(include_str!("input.txt"));
//     let (map, start) = parse(
//         r"#########
// #b.A.@.a#
// #########
// ",
//     );
//     dbg!(reachability(start, &map, &HashSet::new()));
}

fn do_part1(input: &str) {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    let mut rng = thread_rng();
    
    let (map, start_pos) = parse(input);
    let mut tasks = Vec::new();
    let mut best : Option<usize> = None;

    let all_keys: HashSet<_> = map
        .backing
        .iter()
        .filter_map(|t| if let Tile::Key(c) = t { Some(*c) } else { None })
        .collect();
    tasks.push((start_pos, HashSet::new(), all_keys, 0));
    let mut iters = 0;
    while let Some((pos, keys, remaining, traveled)) = tasks.pop() {
        iters += 1;
        if (iters % 1024) == 0 {
            tasks.shuffle(&mut rng);
        }
        if let Some(b) = best {
            if traveled >= b {
                //print!(".");
                continue;
            }
        }

        if remaining.is_empty() {
            if let Some(b) = best {
                if traveled < b {
                    println!("new best route: {} steps", traveled);
                    best = Some(traveled);
                }
            } else {
                println!("new best route: {} steps", traveled);
                best = Some(traveled);
            }
            continue;
        }

        let mut reach : Vec<_> = reachability(pos, &map, &keys).drain().collect();
        reach.sort_by_key(|(_,(_,dist))| *dist);

        for (target, (keypos, dist)) in reach.iter() {
            let mut keys = keys.clone();
            keys.insert(*target);
            let mut remaining = remaining.clone();
            remaining.remove(&target);
            tasks.push((*keypos, keys, remaining, traveled + dist));
        }
    }

    dbg!(best);
}

fn parse(input: &str) -> (Map, Pos) {
    let mut result = Vec::new();
    let mut start_pos = None;
    let mut width = None;
    for (y, line) in input.trim().lines().enumerate() {
        if width.is_none() {
            width = Some(line.trim().len());
        }
        for (x, c) in line.trim().chars().enumerate() {
            result.push(match c {
                '#' => Tile::Wall,
                '.' => Tile::Space,
                c @ 'A'..='Z' => Tile::Door(c.to_ascii_lowercase() as u8),
                c @ 'a'..='z' => Tile::Key(c as u8),
                '@' => {
                    start_pos = Some((x as isize, y as isize));
                    Tile::Space
                }
                x => panic!("unknown map char: {}", x),
            });
        }
    }
    (
        Map {
            backing: result,
            width: width.unwrap(),
        },
        start_pos.unwrap(),
    )
}

#[derive(Eq, PartialEq, Hash)]
enum Tile {
    Wall,
    Space,
    Door(u8),
    Key(u8),
}

trait PosMod {
    fn up(&self) -> Self;
    fn down(&self) -> Self;
    fn left(&self) -> Self;
    fn right(&self) -> Self;
}

impl PosMod for Pos {
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

fn reachability(start: (isize, isize), map: &Map, keys: &HashSet<u8>) -> HashMap<u8, (Pos, usize)> {
    let mut tasks = VecDeque::new();
    struct MinDist {
        backing: Vec<usize>,
        width: usize,
    }
    impl MinDist {
        fn set(&mut self, pos: &Pos, dist: usize) {
            self.backing[pos.1 as usize *self.width+pos.0 as usize] = dist;
        }
        fn get(&self, pos: &Pos) -> usize {
            self.backing[pos.1 as usize *self.width+pos.0 as usize]
        }
    }

    let mut min_dist = MinDist { backing: Vec::new(), width: map.width };
    min_dist.backing.resize(map.backing.len(), usize::max_value());
    min_dist.set(&start, 0);

    let mut result = HashMap::new();
    tasks.push_back((start, 0));
    while let Some((pos, dist)) = tasks.pop_front() {
        for probe in &[pos.up(), pos.down(), pos.left(), pos.right()] {
            let tile = map.get(probe);
            let ok = match tile {
                Some(Tile::Wall) => false,
                Some(Tile::Space) => true,
                Some(Tile::Door(k)) => keys.contains(k),
                Some(Tile::Key(_)) => true,
                None => false, // off da map
            };
            if ok {
                if min_dist.get(probe) < dist + 1 {
                    continue;
                }
                min_dist.set(probe, dist + 1);
                tasks.push_back((*probe, dist + 1));
                if let Some(Tile::Key(k)) = tile {
                    if !keys.contains(k) {
                        result.insert(*k, (*probe, dist + 1));
                    }
                }
            }
        }
    }
    result
}
