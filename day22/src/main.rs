
enum Step {
    NewStack,
    CutN(isize),
    DealIncr(isize),
}

const CUT : &'static str = "cut ";
const DEAL_INCR : &'static str = "deal with increment ";
const DEAL_STACK : &'static str = "deal into new stack";

fn parse(input: &str) -> Vec<Step> {
    input.trim().lines().map(|line| {
        let line = line.trim();
        if line == DEAL_STACK {
            Step::NewStack
        } else if line.starts_with(CUT) {
            Step::CutN(line[CUT.len()..].parse().unwrap())
        } else if line.starts_with(DEAL_INCR) {
            Step::DealIncr(line[DEAL_INCR.len()..].parse().unwrap())
        } else {
            panic!("dunno what to do: {}", line);
        }
    }).collect()
}

fn do_part1(steps: Vec<Step>) {
    const DECK_SIZE : usize = 10_007;
    let mut cards : Vec<_> = (0..(DECK_SIZE as u32)).collect();
    for step in steps.into_iter() {
        cards = match step {
            Step::NewStack => { cards.into_iter().rev().collect() }
            Step::CutN(mut n) => { 
                if n < 0 {
                    n += DECK_SIZE as isize;
                }
                cards[n as usize..].iter().chain(cards[..n as usize].iter()).copied().collect() 
            }
            Step::DealIncr(mut n) => {
                let mut new = vec![0;DECK_SIZE];
                if n < 0 {
                    n += DECK_SIZE as isize;
                }
                for i in 0..DECK_SIZE {
                    new[(i*(n as usize))%DECK_SIZE] = cards[i];
                }
                new
            }
        }
    }
    println!("2019 at idx: {:?}", cards.iter().position(|&t| t == 2019));
}

fn do_part2(steps: Vec<Step>) {   
    const DECK_SIZE : isize = 119_315_717_514_047;
 
    let mut tracked_pos = 2020_isize;
    for step in steps.into_iter().rev() {
        match step {
            Step::NewStack => tracked_pos = DECK_SIZE - tracked_pos - 1,
            Step::CutN(mut n) => {
                // = (tracked_pos + n) mod d
                if n < 0 {
                    n += DECK_SIZE;
                }
                tracked_pos += n;
                if tracked_pos > DECK_SIZE {
                    tracked_pos -= DECK_SIZE;
                }
            }
            Step::DealIncr(mut n) => {
                // = (tracked_pos * n) mod d
                
            }
        }
    }
}

fn main() {
    do_part1(parse(include_str!("input")));
    do_part2(parse(include_str!("input")));
}
