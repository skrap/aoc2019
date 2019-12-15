use std::collections::{HashMap, VecDeque};

fn main() {
    run_part1(include_str!("input.txt"));   
    run_part2(include_str!("input.txt"));
}

fn run_part2(input: &'static str) {
    let ingreds = parse(input);
    let mut high = 1000;
    let target = 1_000_000_000_000;
    while make_fuel(&ingreds, high) <= target {
        high *= 2;
    }
    high *= 2; // go above
    let mut low = high/4;
    loop {
        use std::cmp::Ordering::*;
        let probe =  (high + low) / 2;
        let ore = make_fuel(&ingreds, probe);
        println!("{} fuel takes {:?} ore", probe, ore); 
        match ore.cmp(&target) {
            Greater => high = probe,
            Less => low = probe,
            Equal => { high = probe; break },
        }
        if low + 1 == high {
            break;
        }
    }
    println!("low {} high {}", low, high);
}

fn run_part1(input: &'static str) {
    let ingreds = parse(input);
    println!("num_ore required for 1 fuel: {}", make_fuel(&ingreds, 1));
}

type Ingreds = HashMap<&'static str, ((&'static str, usize), Vec<(&'static str, usize)>)>;
type Inventory = HashMap<&'static str, usize>;

fn make_fuel (ingreds: &Ingreds, num_fuel: usize) -> usize {
    let mut leftovers = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(("FUEL", num_fuel));
    let mut num_ore = 0;

    while let Some((kind, mut num)) = queue.pop_front() {
        //println!("make {} {}", num, kind);
        let left = leftovers.entry(kind).or_insert(0);
        let used = (*left).min(num);
        *left -= used;
        num -= used;

        if kind == "ORE" {
            num_ore += num;
            continue;
        }
        
        if num == 0 {
            continue;
        }

        let (out,ins) = ingreds.get(kind).expect("can't make ingredient");
        let produced_times = (num+out.1-1)/out.1;
        // we need to make some of `kind`:
        for in1 in ins {
            let mut to_do = *in1;
            to_do.1 *= produced_times;
            queue.push_back(to_do);
        }
        *leftovers.entry(kind).or_insert(0) += (out.1*produced_times) - num;
    }

    num_ore
}

fn parse(input: &'static str) -> Ingreds {
    let mut ingreds = HashMap::new();
    for line in input.trim().lines() {
        fn parse1(in1: &str) -> (&str, usize) {
            let mut t = in1.trim().split(' ');
            let quantity = t.next().unwrap().parse::<usize>().unwrap();
            let kind = t.next().unwrap();
            (kind, quantity)
        }
        let mut halfs = line.split("=>");
        let (ins,out) = (halfs.next().unwrap().trim(), halfs.next().unwrap().trim());
        let out = parse1(out);
        let ins :Vec<_> = ins.split(',').map(|t| parse1(t)).collect();
        ingreds.insert(out.0, (out,ins));
    }
    ingreds
}


