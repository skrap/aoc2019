#[derive(Clone, Copy)]
enum Step {
    NewStack,
    CutN(isize),
    DealIncr(isize),
}

const CUT: &'static str = "cut ";
const DEAL_INCR: &'static str = "deal with increment ";
const DEAL_STACK: &'static str = "deal into new stack";

fn parse(input: &str) -> Vec<Step> {
    input
        .trim()
        .lines()
        .map(|line| {
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
        })
        .collect()
}

fn do_part1(steps: Vec<Step>) -> Vec<u32> {
    const DECK_SIZE: usize = 10_007;
    let mut cards: Vec<_> = (0..(DECK_SIZE as u32)).collect();
    for step in steps.into_iter() {
        cards = match step {
            Step::NewStack => cards.into_iter().rev().collect(),
            Step::CutN(mut n) => {
                if n < 0 {
                    n += DECK_SIZE as isize;
                }
                cards[n as usize..]
                    .iter()
                    .chain(cards[..n as usize].iter())
                    .copied()
                    .collect()
            }
            Step::DealIncr(mut n) => {
                let mut new = vec![0; DECK_SIZE];
                if n < 0 {
                    n += DECK_SIZE as isize;
                }
                for i in 0..DECK_SIZE {
                    new[(i * (n as usize)) % DECK_SIZE] = cards[i];
                }
                new
            }
        }
    }
    cards
}

fn mul_inv(a: isize, n: isize) -> isize {
    let mut r = [n.max(a), n.min(a)];
    let mut s = [1, 0];
    let mut t = [0, 1];
    loop {
        let qi = r[0] / r[1];
        let ri = r[0] - qi * r[1];
        if ri == 0 {
            break;
        }
        r = [r[1], r[0] - qi * r[1]];
        s = [s[1], s[0] - qi * s[1]];
        t = [t[1], t[0] - qi * t[1]];
    }
    t[1]
}

#[test]
fn test_mul_inv() {
    assert_eq!(mul_inv(3, 10) + 10, 7);
}

fn rev_track(steps: &[Step], tracked_pos: isize, deck_size: i128) -> isize {
    let deck_size = deck_size as isize;
    let mut tracked_pos = tracked_pos as i128;
    for step in steps.into_iter().rev() {
        match step {
            Step::NewStack => {
                // = (- tracked_pos - 1) mod d
                tracked_pos = -1 - tracked_pos
            }
            Step::CutN(n) => {
                // = (tracked_pos + n) mod d
                tracked_pos += *n as i128;
            }
            Step::DealIncr(n) => {
                // = (tracked_pos * n^-1) mod d
                tracked_pos *= mul_inv(*n, deck_size) as i128;
            }
        }
        tracked_pos %= deck_size as i128;
        //println!("tracked_pos: {}", tracked_pos);
    }
    tracked_pos as isize
}

#[test]
fn test_rev_full() {
    assert_eq!(
        2019,
        (rev_track(&parse(include_str!("input")), 3939, 10_007) + 10_007) % 10_007
    );
}

#[test]
fn test_rev_newstack() {
    let step = vec![Step::NewStack];
    for i in 0..10 {
        assert_eq!(9 - i, (rev_track(&step, i, 10) + 100) % 10);
    }
}

#[test]
fn test_rev_cutn() {
    let step = vec![Step::CutN(3)];
    let ans = [3, 4, 5, 6, 7, 8, 9, 0, 1, 2];
    for i in 0..10 {
        assert_eq!(ans[i as usize], (rev_track(&step, i, 10) + 100) % 10);
    }
}

#[test]
fn test_rev_cutn_neg() {
    let step = vec![Step::CutN(-4)];
    let ans = [6, 7, 8, 9, 0, 1, 2, 3, 4, 5];
    for i in 0..10 {
        assert_eq!(ans[i as usize], (rev_track(&step, i, 10) + 100) % 10);
    }
}

#[test]
fn test_rev_dealincr() {
    let step = vec![Step::DealIncr(3)];
    let ans = [0, 7, 4, 1, 8, 5, 2, 9, 6, 3];
    for i in 0..10 {
        assert_eq!(ans[i as usize], (rev_track(&step, i, 10) + 100) % 10);
    }
}


fn modular_pow(mut b: i128, mut e: i128, m: i128) -> isize {
    if m == 1 {
        return 0;
    }
    let mut r = 1;
    b %= m;
    while e > 0 {
        if e % 2 == 1 {
            r = (r * b) % m;
        }
        e >>= 1;
        b = (b * b) % m;
    }
    r as isize
}
#[test]
fn test_mod_pow() {
    assert_eq!(modular_pow(2, 31, 93409), 2_isize.pow(31) % 93409);
}

fn main() {
    let parsed = parse(include_str!("input"));
    let cards = do_part1(parsed.clone());
    println!("2019 at idx: {:?}", cards.iter().position(|&t| t == 2019));

    let big_deck_size = 119315717514047_i128;
    let shuffle_count = 101741582076661_i128;
    let cut = rev_track(&parsed, 0, big_deck_size) as i128;
    let incr = rev_track(&parsed, 1, big_deck_size) as i128 - cut;
    println!("{} + {}x", cut, incr);
    
    let f1 = (cut,incr);
    let ffx = |outer: (i128, i128), inner: (i128, i128)| ((outer.0+outer.1*inner.0)%big_deck_size, (outer.1*inner.1)%big_deck_size);
    
    let automatic = |needle, mut times| {
        let mut f = f1;
        let mut f_shuffle = (0,1);
        while times > 0 {
            if times & 1 == 1 {
                f_shuffle = ffx(f,f_shuffle);
            }
            times >>= 1;
            f = ffx(f,f);
        }
        ((f_shuffle.0+f_shuffle.1*needle)+big_deck_size)%big_deck_size
    };

    let manual = |needle, times| {
        let mut needle = needle as isize;
        for _ in 0..times {
            needle = rev_track(&parsed, needle, big_deck_size);
        }
        needle as i128
    };

    for times in 0..5 {
        let needle = 0_i128;
        println!("{} man: {}, auto: {}", times, manual(needle, times), automatic(needle,times));
    }

    println!("Final answer: {}", automatic(2020, shuffle_count)+big_deck_size);
}
