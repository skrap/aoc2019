fn to_grid(input: &str) -> u32 {
    let mut out = 0;
    for (bit, i) in input
        .chars()
        .filter_map(|c| match c {
            '#' => Some(1),
            '.' => Some(0),
            _ => None,
        })
        .enumerate()
    {
        out |= i << bit;
    }
    out
}

#[test]
fn test_grid_rating() {
    let input = ".....
    .....
    .....
    #....
    .#...
    ";
    assert_eq!(to_grid(input), 2129920);
}

fn next(grid: u32) -> u32 {
    let width = 5;
    let mut out = 0;
    for bit in 0..(width * 5) {
        let norm = if bit < 5 {
            grid << (5 - bit)
        } else {
            grid >> (bit - 5)
        };
        let mask = match bit {
            // left edge
            0 | 5 | 10 | 15 | 20 => 0b10001000001,
            // right edge
            4 | 9 | 14 | 19 | 24 => 0b10000010001,
            _ => 0b10001010001,
        };
        out |= match (norm & mask).count_ones() {
            1 => 1,
            2 => (!norm & (1 << 5)) >> 5,
            _ => 0,
        } << bit;
    }
    out
}

fn next_single_pt2(outer: u32, grid: u32, inner: u32) -> u32 {
    let width = 5;
    let mut out = 0;
    for bit in 0..(width * 5) {
        if bit == 12 {
            continue; // here there be dragons
        }
        let norm = if bit < 5 {
            grid << (5 - bit)
        } else {
            grid >> (bit - 5)
        };
        let mask = match bit {
            // left edge
            _ if (bit % 5) == 0 => 0b10001000001,
            // right edge
            _ if (bit % 5) == 4 => 0b10000010001,
            _ => 0b10001010001,
        };
        let mut adjacent = (norm & mask).count_ones();
        if bit < 5 {
            adjacent += (outer >> 7) & 1;
        }
        if (bit % 5) == 0 {
            adjacent += (outer >> 11) & 1;
        }
        if bit >= 20 {
            adjacent += (outer >> 17) & 1;
        }
        if (bit % 5) == 4 {
            adjacent += (outer >> 13) & 1;
        }
        if bit == 7 {
            adjacent += (inner & 0b11111).count_ones();
        }
        if bit == 11 {
            adjacent += (inner & (0b0000100001000010000100001)).count_ones();
        }
        if bit == 13 {
            adjacent += (inner & (0b1000010000100001000010000)).count_ones();
        }
        if bit == 17 {
            adjacent += (inner & (0b11111 << 20)).count_ones();
        }
        out |= match adjacent {
            1 => 1,
            2 => (!norm & (1 << 5)) >> 5,
            _ => 0,
        } << bit;
    }
    out
}

fn next_pt2(levels: &[u32]) -> Vec<u32> {
    let mut out = vec![next_single_pt2(0, levels[0], levels[1])];
    for level in levels.windows(3) {
        out.push(next_single_pt2(level[0], level[1], level[2]));
    }
    out.push(next_single_pt2(
        levels[levels.len() - 2],
        levels[levels.len() - 1],
        0,
    ));
    out
}

#[test]
fn test_next_pt2() {
    let mut levels = vec![0; 11];
    levels[5] = to_grid(
        "....#
    #..#.
    #.?##
    ..#..
    #....
    ",
    );
    for _min in 0..10 {
        levels = next_pt2(&levels);
    }
    let expected: Vec<_> = vec![
        "..#..
        .#.#.
        ..?.#
        .#.#.
        ..#..",
        "...#.
        ...##
        ..?..
        ...##
        ...#.
        ",
        "#.#..
        .#...
        ..?..
        .#...
        #.#..",
        ".#.##
        ....#
        ..?.#
        ...##
        .###.
        ",
        "#..##
        ...##
        ..?..
        ...#.
        .####",
        ".#...
        .#.##
        .#?..
        .....
        .....",
        ".##..
        #..##
        ..?.#
        ##.##
        #####",
        "###..
        ##.#.
        #.?..
        .#.##
        #.#..",
        "..###
        .....
        #.?..
        #....
        #...#",
        ".###.
        #..#.
        #.?..
        ##.#.
        .....",
        "####.
        #..#.
        #.?#.
        ####.
        .....",
    ]
    .into_iter()
    .map(to_grid)
    .collect();

    assert_eq!(levels.iter().map(|&l| l.count_ones()).sum::<u32>(), 99u32);

    assert_eq!(&expected, &levels);
}

#[test]
fn test_next() {
    let grid0 = to_grid(
        "....#
    #..#.
    #..##
    ..#..
    #....",
    );
    let grid1 = to_grid(
        "#..#.
    ####.
    ###.#
    ##.##
    .##..",
    );
    assert_eq!(next(grid0), grid1);
}

fn main() {
    let input = "##.#.
    .##..
    ##.#.
    .####
    ###..";
    let mut grid = to_grid(input);
    let mut seen = std::collections::HashSet::new();
    while !seen.contains(&grid) {
        seen.insert(grid);
        grid = next(grid);
    }
    println!("duplicate found: {}", grid);
}
