use itertools::Itertools;

#[test]
fn test_mod() {
    assert_eq!((-17 % 10isize).abs(), 7_isize);
}

fn dig(i: isize) -> isize {
    (i % 10).abs()
}

use std::iter::repeat;

fn fft(input: Vec<isize>) -> Vec<isize> {
    let mut next_numbers = Vec::new();
    for digit in 0..input.len() {
        let mut pat = repeat(0)
            .take(digit + 1)
            .chain(repeat(1).take(digit + 1))
            .chain(repeat(0).take(digit + 1))
            .chain(repeat(-1).take(digit + 1))
            .cycle();
        pat.next();
        next_numbers.push(dig(input.iter().zip(pat).map(|(n, p)| n * p).sum()));
    }
    next_numbers
}

fn parse(input: &str) -> Vec<isize> {
    input.trim()
    .chars()
    .map(|c| c.to_digit(10).unwrap() as isize)
    .collect()
}

fn main() {
    let mut numbers: Vec<isize> = parse(include_str!("input.txt"));
    dbg!(numbers.len());
    let print_it = |it: &[isize]| {
        println!("{}", it.iter().map(|&d| std::char::from_digit(d as u32, 10).unwrap()).collect::<String>());
    };
    for _round in 0..100 {
        numbers = fft(numbers);
    }
    print_it(&numbers[0..80]);

    let numbers: Vec<isize> = 
        parse("80871224585914546619083218645595");
    print_it(&fft(numbers.clone()));
    let mut bigger = numbers.clone();
    bigger.extend_from_slice(&numbers);
    print_it(&fft(bigger));
}
