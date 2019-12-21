#[test]
fn test_mod() {
    assert_eq!((-17 % 10isize).abs(), 7_isize);
}

fn dig(i: isize) -> u8 {
    (i % 10).abs() as u8
}

use std::iter::repeat;

fn fft(input: &[u8]) -> Vec<u8> {
    let mut next_numbers = Vec::new();
    for digit in 0..input.len() {
        let mut pat = repeat(0)
            .take(digit + 1)
            .chain(repeat(1).take(digit + 1))
            .chain(repeat(0).take(digit + 1))
            .chain(repeat(-1).take(digit + 1))
            .cycle();
        pat.next();
        next_numbers.push(dig(input.iter().zip(pat).map(|(n, p)| (*n as isize) * p).sum()));
    }
    next_numbers
}

fn parse(input: &str) -> Vec<u8> {
    input.trim()
    .chars()
    .map(|c| c.to_digit(10).unwrap() as u8)
    .collect()
}

#[allow(dead_code)]
fn fft2(i: usize, round: usize, input: &[u8], maxlen: usize) -> u8 {
    let make_pat = |j| {
        let mut pat = repeat(0)
        .take(j + 1)
        .chain(repeat(1).take(j + 1))
        .chain(repeat(0).take(j + 1))
        .chain(repeat(-1).take(j + 1))
        .cycle();
        pat.next();
        pat
    };

    if round == 0 {
        input[i%input.len()]
    } else if i + 1 == maxlen {
        fft2(i,round-1,input,maxlen)
    } else {
        // only ask about stuff > i or lower rounds
        let mut sum = fft2(i+1,round,input,maxlen) as isize;
        let pat_i = make_pat(i);
        let pat_next = make_pat(i+1);
        for (xi, (ifactor, i1factor)) in pat_i.zip(pat_next).enumerate() {
            if ifactor != i1factor {
                sum += (i1factor-ifactor)*(fft2(xi, round-1, input, maxlen) as isize);
            }
        }
        dig(sum)
    }
}

fn main() {
    let mut numbers: Vec<u8> = parse(include_str!("input.txt"));
    dbg!(numbers.len());
    let print_it = |it: &[u8]| {
        println!("{}", it.iter().map(|&d| std::char::from_digit(d as u32, 10).unwrap()).collect::<String>());
    };
    for _round in 0..1 {
        numbers = fft(&numbers);
    }
    print!("Part 1 solution: ");
    print_it(&numbers[0..80]);

    let teststr = include_str!("input.txt"); //"03036732577212944063491565474664";
    let input = parse(teststr); //;
    let offset = teststr[0..7].parse::<usize>().unwrap();
    let maxlen = input.len()*10_000;
    assert!(offset*2>maxlen);
    let mut vals = Vec::with_capacity(maxlen-offset);
    let mut i = offset%input.len();
    while vals.len() < (maxlen-offset) {
        vals.push(input[i]);
        i += 1;
        if i >= input.len() { i = 0 } 
    }
    for _ in 0..100 {
        for i in (0..(vals.len()-1)).rev() {
            vals[i] = dig(vals[i] as isize + vals[i+1] as isize);
        }
    }

    println!("Part 2 solution: {}", vals[0..8].iter().map(|&d| std::char::from_digit(d as u32, 10).unwrap()).collect::<String>());
}
