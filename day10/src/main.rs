use std::collections::VecDeque;
type Pt = (isize, isize);

struct Map<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Map<T>
where
    T: Copy,
{
    fn at(&self, pos: &Pt) -> T {
        assert!(self.contains(pos));
        let pos = (pos.0 as usize, pos.1 as usize);
        self.data[pos.0 + pos.1 * self.width]
    }

    fn contains(&self, pos: &Pt) -> bool {
        pos.0 >= 0 && (pos.0 as usize) < self.width && pos.1 >= 0 && (pos.1 as usize) < self.height
    }

    fn set(&mut self, pos: &Pt, val: T) {
        let pos = (pos.0 as usize, pos.1 as usize);
        self.data[pos.0 + pos.1 * self.width] = val;
    }
}

fn parse(input: &str) -> Map<bool> {
    let width = input.trim().lines().next().unwrap().trim().len();
    let data: Vec<bool> = input
        .trim()
        .lines()
        .map(|line| line.trim().chars().map(|ch| ch == '#'))
        .flatten()
        .collect();
    let height = data.len() / width;
    Map {
        width,
        height,
        data,
    }
}

fn delta(a: &Pt, b: &Pt) -> Pt {
    ((a.0 - b.0), (a.1 - b.1))
}

fn ast_pos(map: &Map<bool>) -> Vec<Pt> {
    let mut ast_pos = Vec::new();
    for (i, val) in map.data.iter().enumerate() {
        if *val {
            let y = i / map.width;
            let pos = ((i - y * map.width) as isize, y as isize);
            ast_pos.push(pos);
        }
    }
    ast_pos
}

fn radial_pos(center: Pt, map: &Map<bool>) -> Vec<Pt> {
    let mut result = Vec::new();
    for x in 0..map.width {
        for y in 0..map.height {
            let pos = (x as isize, y as isize);
            if pos != center {
                result.push(pos);
            }
        }
    }
    result.sort_by(|a, b| {
        let keyfn = |(x, y)| {
            let (x, mut y) = ((x - center.0) as f64, (center.1 - y) as f64);
            let ph = if x < 0.0 { y *= -1.0; std::f64::consts::PI } else { 0.0 };
            (y / (x * x + y * y).sqrt()).acos() + ph
        };
        keyfn(*a).partial_cmp(&keyfn(*b)).unwrap()
    });
    result
}

fn pt1(map: &str) -> (Pt, usize) {
    let map = parse(map);
    let asts = ast_pos(&map);
    let base = asts
        .iter()
        .max_by_key(|&&base| make_occl_rays(base, &map).len())
        .unwrap();
    (*base, make_occl_rays(*base, &map).len())
}

fn make_occl_rays(base: Pt, map: &Map<bool>) -> Vec<VecDeque<Pt>> {
    let ast_pos = radial_pos(base, &map);
    let mut occls = Map {
        height: map.height,
        width: map.width,
        data: std::iter::repeat(false).take(map.data.len()).collect(),
    };

    let mut occl_rays = Vec::new();
    for pos in ast_pos.iter() {
        if occls.at(pos) {
            continue;
        }
        if *pos == base {
            continue;
        }
        let mut this_ray = VecDeque::new();
        let mut diff = delta(&pos, &base);
        // brute force simplification of fraction.
        for divby in 2..(map.height.max(map.width) as isize) {
            while (diff.0 % divby) == 0 && (diff.1 % divby) == 0 {
                diff.0 /= divby;
                diff.1 /= divby;
            }
        }

        let mut pos = base;
        while occls.contains(&pos) {
            if pos != base && !occls.at(&pos) && map.at(&pos) {
                this_ray.push_back(pos);
            }
            occls.set(&pos, true);
            pos.0 += diff.0;
            pos.1 += diff.1;
        }
        // println!("Center {:?} Ray: {:?}", base, &this_ray);
        if !this_ray.is_empty() {
            occl_rays.push(this_ray);
        }
    }
    occl_rays
}

fn pt2(base: Pt, map: &str, ast_target: usize) -> Pt {
    let map = parse(map);
    let mut occl_rays = make_occl_rays(base, &map);
    let mut laser_count = 0;
    loop {
        'ray: for occl_ray in occl_rays.iter_mut() {
            if let Some(pos) = occl_ray.pop_front() {
                laser_count += 1;
                //  println!("asteroid {} is at {:?}", laser_count, pos);
                if laser_count == ast_target {
                    return pos;
                }
                continue 'ray;
            }
        }
    }
}

fn main() {
    let pt1 = pt1(INPUT_MAP);
    println!("Part 1: {:?}", &pt1);
    println!("Part 2: {:?}", pt2(pt1.0, INPUT_MAP, 200));
}

const INPUT_MAP: &str = include_str!("input.txt");

#[test]
fn test_pt2() {
    let test_pt2_map = r".#....#####...#..
    ##...##.#####..##
    ##...#...#.#####.
    ..#.....X...###..
    ..#.#.....#....##";
    assert_eq!(pt2((8,3), test_pt2_map, 9*4), (14,3));

    let test_map4 = r".#..##.###...#######
    ##.############..##.
    .#.######.########.#
    .###.#######.####.#.
    #####.##.#.##.###.##
    ..#####..#.#########
    ####################
    #.####....###.#.#.##
    ##.#################
    #####.##.###..####..
    ..######..##.#######
    ####.##.####...##..#
    .#####..#.######.###
    ##...#.##########...
    #.##########.#######
    .####.#.###.###.#.##
    ....##.##.###..#####
    .#.#.###########.###
    #.#.#.#####.####.###
    ###.##.####.##.#..##";
    let map = test_map4;
    let pt1 = pt1(map);
    assert_eq!(pt2(pt1.0, map, 200), (8, 2));
}

#[test]
fn test_canned() {
    let ex_map = r".#..#
.....
#####
....#
...##";
    let test_map1 = r"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";
    let test_map2 = r"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";
    let test_map3 = r".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";
    let test_map4 = r".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
    assert_eq!(pt1(ex_map), ((3, 4), 8));
    assert_eq!(pt1(test_map1), ((5, 8), 33));
    assert_eq!(pt1(test_map2), ((1, 2), 35));
    assert_eq!(pt1(test_map3), ((6, 3), 41));
    assert_eq!(pt1(test_map4), ((11, 13), 210));
}
