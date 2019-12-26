use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

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
    fn set(&mut self, pos: &Pos, tile: Tile) {
        self.backing[pos.1 as usize * self.width + pos.0 as usize] = tile;
    }
}

fn main() {
    // do_part1(
    //     r"########################
    // #...............b.C.D.f#
    // #.######################
    // #.....@.a.B.c.d.A.e.F.g#
    // ########################
    // ",
    // );
    // do_part1(
    //     r"#################
    // #i.G..c...e..H.p#
    // ########.########
    // #j.A..b...f..D.o#
    // ########@########
    // #k.E..a...g..B.n#
    // ########.########
    // #l.F..d...h..C.m#
    // #################",
    // );
    // do_part1(
    //     r"########################
    // #@..............ac.GI.b#
    // ###d#e#f################
    // ###A#B#C################
    // ###g#h#i################
    // ########################
    // ",
    // );
    // do_part1(include_str!("input.txt"));
    do_part2(include_str!("input.txt"));
}

fn do_part2(input: &str) {
    let (mut map, mid_pos) = parse(input);
    let deps = make_deps(mid_pos, &map);
    let (&key_min, &key_max) = deps.keys().minmax().into_option().unwrap();
    let key_mask = |k: u8| -> u32 { 1 << (k - key_min) as u32 };

    map.set(&mid_pos, Tile::Wall);
    map.set(&mid_pos.up(), Tile::Wall);
    map.set(&mid_pos.down(), Tile::Wall);
    map.set(&mid_pos.left(), Tile::Wall);
    map.set(&mid_pos.right(), Tile::Wall);
    let start_1 = mid_pos.up().left();
    let start_2 = mid_pos.up().right();
    let start_3 = mid_pos.down().left();
    let start_4 = mid_pos.down().right();

    let mut tasks: BinaryHeap<(u32, u32, [Pos; 4], usize)> = BinaryHeap::new();
    let mut all_keys = 0;
    for key in deps.keys() {
        all_keys |= key_mask(*key);
    }

    let dep_masks = deps
        .iter()
        .map(|(k, (_, _, key_deps))| {
            (k, {
                let mut all_keys = 0;
                for key in key_deps {
                    all_keys |= key_mask(*key);
                }
                all_keys
            })
        })
        .collect::<HashMap<_, _>>();
    let mut iters = 0;
    let mut best_soln = None;
    let mut best: HashMap<(u32, Pos), usize> = HashMap::new();
    tasks.push((0, 0, [start_1, start_2, start_3, start_4], 0));
    while let Some((_, have, bots_pos, traveled)) = tasks.pop() {
        iters += 1;
        if (iters % 10000) == 0 {
            println!("iter {} traveled {} have {:02}/{:02}", iters, traveled, have.count_ones(), all_keys.count_ones());
        }
        if let Some(soln) = best_soln {
            if traveled >= soln {
                continue;
            }
        }
        let other_is_better = |bot_pos: &Pos| {
            if let Some(&memoed) = best.get(&(have, *bot_pos)) {
                if memoed <= traveled {
                    return true;
                }
            }
            false
        };
        if bots_pos.iter().all(other_is_better) {
            continue;
        }
        for bot_pos in &bots_pos {
            best.insert((have, *bot_pos), traveled);
        }
        if all_keys == have {
            println!("New best: {}", traveled);
            best_soln = Some(traveled);
            continue;
        }

        let get_steps = |pos: &Pos| vec![pos.up(), pos.down(), pos.left(), pos.right(), *pos];
        'step: for step in bots_pos
            .iter()
            .map(|one_bot_pos| get_steps(one_bot_pos).into_iter())
            .multi_cartesian_product()
        {
            let tiles = step.iter().map(|s| map.get(s));
            let mut new_have = have;
            for tile in tiles {
                match tile {
                    Some(Tile::Wall) => continue 'step,
                    Some(Tile::Space) => (),
                    Some(Tile::Door(k)) => {
                        if new_have & (key_mask(*k)) == 0 {
                            continue 'step;
                        }
                    }
                    Some(Tile::Key(k)) => {
                        //println!("got key {}", k);
                        new_have |= key_mask(*k)
                    }
                    None => continue 'step, // off da map
                }
            }
            use std::convert::TryInto;
            tasks.push((
                new_have.count_ones(),
                new_have,
                step[..].try_into().unwrap(),
                traveled + 1,
            ));
        }
    }
    println!("done {} tasks", iters);
    let winner = best
        .iter()
        .filter(|((needs, _), _)| *needs == 0)
        .min_by_key(|(_, &traveled)| traveled);
    println!("best path {:?} steps", winner);
}

fn do_part1(input: &str) {
    let (map, start_pos) = parse(input);

    let deps = make_deps(start_pos, &map);
    let (&key_min, &key_max) = deps.keys().minmax().into_option().unwrap();
    let key_mask = |k: u8| -> u32 { 1 << (k - key_min) as u32 };

    let mut tasks = BinaryHeap::new();
    let mut all_keys = 0;
    for key in deps.keys() {
        all_keys |= key_mask(*key);
    }

    let dep_masks = deps
        .iter()
        .map(|(k, (_, _, key_deps))| {
            (k, {
                let mut all_keys = 0;
                for key in key_deps {
                    all_keys |= key_mask(*key);
                }
                all_keys
            })
        })
        .collect::<HashMap<_, _>>();

    // precompute all the key-to-key distances
    let mut dists = Vec::new();
    let num_keys = (key_max - key_min + 1) as usize;
    dists.resize(num_keys * num_keys, 0);
    for (from_key, (pos, from_start_dist, key_deps)) in deps.iter() {
        if key_deps.is_empty() {
            tasks.push((
                0,
                usize::max_value(),
                key_mask(*from_key),
                all_keys & !key_mask(*from_key),
                *from_key,
                *from_start_dist,
                vec![*from_key],
            ));
        }
        let key_deps = make_deps(*pos, &map);
        for (to_key, (_, key_dist, _)) in key_deps.iter() {
            dists[(*from_key - key_min) as usize * num_keys + (*to_key - key_min) as usize] =
                *key_dist;
        }
    }
    let get_dist = move |from_key, to_key| -> usize {
        dists[(from_key - key_min) as usize * num_keys + (to_key - key_min) as usize]
    };

    let mut iters = 0;
    let mut best = HashMap::new();
    while let Some((_, _, have, need, at_key, traveled, path)) = tasks.pop() {
        iters += 1;

        if let Some(&(memoed, _)) = best.get(&(need, at_key)) {
            if memoed <= traveled {
                continue;
            }
        }
        best.insert((need, at_key), (traveled, path.clone()));
        if need == 0 {
            println!(
                "New best: {}, {}",
                traveled,
                std::str::from_utf8(&path).unwrap()
            );
            continue;
        }

        for key in key_min..=key_max {
            let mask = key_mask(key);
            if (need & mask) > 0 {
                // key is needed
                let need_deps = dep_masks.get(&key).unwrap();
                if (need_deps & !have) == 0 {
                    // we can get `key`
                    let key_key_dist = get_dist(at_key, key);
                    let new_traveled = traveled + key_key_dist;
                    let new_need = need & !mask;
                    let new_have = have | mask;
                    let mut path = path.clone();
                    path.push(key);
                    tasks.push((
                        path.len(),
                        usize::max_value() - new_traveled, // best we could ever do.
                        new_have,
                        new_need,
                        key,
                        new_traveled,
                        path,
                    ));
                }
            }
        }
    }
    println!("done {} tasks", iters);
    let (_, winner) = best
        .iter()
        .filter(|((needs, _), _)| *needs == 0)
        .min_by_key(|(_, (traveled, _))| traveled)
        .unwrap();
    println!(
        "best path {}, {} steps",
        std::str::from_utf8(&winner.1).unwrap(),
        winner.0
    );
}

fn do_part1_old(input: &str) {
    let (map, start_pos) = parse(input);

    let deps = make_deps(start_pos, &map);
    let mut tasks = BinaryHeap::new();
    let mut best = None;

    let all_keys: HashSet<_> = map
        .backing
        .iter()
        .filter_map(|t| if let Tile::Key(c) = t { Some(*c) } else { None })
        .collect();

    #[derive(Eq, PartialEq)]
    struct Task(Pos, Vec<u8>, usize);
    impl Task {
        fn score(&self) -> isize {
            (self.1.len() * 128) as isize - self.2 as isize
        }
    }
    impl Ord for Task {
        fn cmp(&self, other: &Self) -> Ordering {
            self.score().cmp(&other.score())
        }
    }
    impl PartialOrd for Task {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    tasks.push(Task(start_pos, Vec::new(), 0));
    let mut iters = 0;
    while let Some(task) = tasks.pop() {
        let score = task.score();
        let Task(pos, keys, traveled) = task;
        iters += 1;
        if let Some(dist) = best {
            if dist <= traveled {
                continue;
            }
        }

        //println!("{}: {}",traveled,std::str::from_utf8(&keys).unwrap());
        if keys.len() == all_keys.len() {
            best = Some(traveled);
            println!("new best route: {} steps", traveled);
            continue;
        }

        let mut reach: Vec<_> = reachability(pos, &map, &keys).drain().collect();
        reach.sort_by_key(|(_, (_, dist))| *dist);

        for (target, (keypos, dist)) in reach.iter() {
            let mut keys = keys.clone();
            keys.push(*target);
            tasks.push(Task(*keypos, keys, traveled + dist));
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

fn reachability(start: (isize, isize), map: &Map, keys: &[u8]) -> HashMap<u8, (Pos, usize)> {
    let mut tasks = VecDeque::new();

    let mut min_dist = MinDist {
        backing: Vec::new(),
        width: map.width,
    };
    min_dist
        .backing
        .resize(map.backing.len(), usize::max_value());
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

struct MinDist {
    backing: Vec<usize>,
    width: usize,
}
impl MinDist {
    fn set(&mut self, pos: &Pos, dist: usize) {
        self.backing[pos.1 as usize * self.width + pos.0 as usize] = dist;
    }
    fn get(&self, pos: &Pos) -> usize {
        self.backing[pos.1 as usize * self.width + pos.0 as usize]
    }
}

fn make_deps(start: (isize, isize), map: &Map) -> HashMap<u8, (Pos, usize, Vec<u8>)> {
    let mut tasks = VecDeque::new();

    let mut min_dist = MinDist {
        backing: Vec::new(),
        width: map.width,
    };
    min_dist
        .backing
        .resize(map.backing.len(), usize::max_value());
    min_dist.set(&start, 0);

    let mut result: HashMap<u8, (Pos, usize, Vec<u8>)> = HashMap::new();
    tasks.push_back((start, 0, vec![]));
    while let Some((pos, dist, deps)) = tasks.pop_front() {
        for probe in &[pos.up(), pos.down(), pos.left(), pos.right()] {
            let tile = map.get(probe);
            let mut deps = deps.clone();
            let ok = match tile {
                Some(Tile::Wall) => false,
                Some(Tile::Space) => true,
                Some(Tile::Door(k)) => {
                    deps.push(*k);
                    true
                }
                Some(Tile::Key(_)) => true,
                None => false, // off da map
            };
            if ok {
                let min = min_dist.get(probe);
                if min < dist + 1 {
                    continue;
                }
                min_dist.set(probe, dist + 1);
                if let Some(Tile::Key(k)) = tile {
                    if min == dist + 1 {
                        if let Some((_, _, min_deps)) = result.get(k) {
                            if &deps != min_deps {
                                println!(
                                    "multiple routes to {}: {} vs {}",
                                    *k,
                                    std::str::from_utf8(&deps).unwrap(),
                                    std::str::from_utf8(&min_deps).unwrap()
                                );
                            }
                        }
                    }
                    result.insert(*k, (*probe, dist + 1, deps.clone()));
                }
                tasks.push_back((*probe, dist + 1, deps));
            }
        }
    }
    result
}
