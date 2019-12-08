fn main() {
    part1();
    part2();
}

fn part1() {
    let data = include_str!("input.txt").trim().as_bytes();
    let answer = data[..]
        .chunks(25 * 6)
        .min_by_key(|chunk| chunk.iter().filter(|&&b| b == b'0').count())
        .map(|chunk| {
            chunk.iter().filter(|&&b| b == b'1').count()
                * chunk.iter().filter(|&&b| b == b'2').count()
        })
        .unwrap();
    println!("part 1 answer: {}", answer);
}

fn part2() {
    let mut image = [0; 25 * 6];
    let data = include_str!("input.txt").trim().as_bytes();
    for chunk in data[..].rchunks(25 * 6) {
        for (px, layer) in image.iter_mut().zip(chunk.iter()) {
            if *layer == b'0' || *layer == b'1' {
                *px = *layer;
            }
        }
    }
    for row in image[..].chunks(25) {
        use std::iter::FromIterator;
        let row = String::from_iter(row.iter().map(|&b| if b == b'1' { '*' } else { ' ' }));
        println!("{}", row);
    }
}
