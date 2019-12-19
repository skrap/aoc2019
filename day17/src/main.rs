use std::collections::VecDeque;

#[derive(Clone)]
struct IntComp {
    mem: Vec<isize>,
    input: VecDeque<isize>,
    output: VecDeque<isize>,
    pc: usize,
    rel_base: isize,
}

#[derive(Debug)]
enum State {
    Run,
    NeedsInput,
    Halt,
}

enum Arg {
    Imm(isize),
    Pos(usize),
    Rel(isize, isize),
}

impl Arg {
    fn load(&self, mem: &[isize]) -> isize {
        match self {
            Arg::Imm(val) => *val,
            Arg::Pos(val) => mem[*val as usize],
            Arg::Rel(val, base) => mem[(*val + *base) as usize],
        }
    }

    fn store(&self, val: isize, mem: &mut [isize]) {
        match self {
            Arg::Pos(pos) => mem[*pos] = val,
            Arg::Rel(pos, rel_base) => mem[(*pos + *rel_base) as usize] = val,
            Arg::Imm(..) => panic!("Can't store to an immediate."),
        }
    }
}

enum Instr {
    Add(Arg, Arg, Arg),
    Mul(Arg, Arg, Arg),
    Input(Arg),
    Output(Arg),
    JumpIf(Arg, Arg),
    JumpUnless(Arg, Arg),
    LessThan(Arg, Arg, Arg),
    Equals(Arg, Arg, Arg),
    SetRelBase(Arg),
    Halt,
}

impl Instr {
    fn execute(&self, comp: &mut IntComp) -> State {
        use Instr::*;
        match self {
            Add(arg1, arg2, dest) => {
                dest.store(arg1.load(&comp.mem) + arg2.load(&comp.mem), &mut comp.mem)
            }
            Mul(arg1, arg2, dest) => {
                dest.store(arg1.load(&comp.mem) * arg2.load(&comp.mem), &mut comp.mem)
            }
            Input(arg) => arg.store(comp.input.pop_front().unwrap(), &mut comp.mem),
            Output(arg) => comp.output.push_back(arg.load(&comp.mem)),
            JumpIf(arg, new_pc) => {
                if arg.load(&comp.mem) != 0 {
                    comp.pc = new_pc.load(&comp.mem) as usize;
                }
            }
            JumpUnless(arg, new_pc) => {
                if arg.load(&comp.mem) == 0 {
                    comp.pc = new_pc.load(&comp.mem) as usize;
                }
            }
            LessThan(arg1, arg2, dest) => dest.store(
                if arg1.load(&comp.mem) < arg2.load(&comp.mem) {
                    1
                } else {
                    0
                },
                &mut comp.mem,
            ),
            Equals(arg1, arg2, dest) => dest.store(
                if arg1.load(&comp.mem) == arg2.load(&comp.mem) {
                    1
                } else {
                    0
                },
                &mut comp.mem,
            ),
            SetRelBase(arg) => comp.rel_base += arg.load(&comp.mem),
            Halt => (),
        };

        if let Halt = self {
            State::Halt
        } else {
            State::Run
        }
    }
}

impl IntComp {
    fn new(mem: Vec<isize>, input: &[isize]) -> IntComp {
        IntComp {
            mem,
            input: input.iter().cloned().collect(),
            output: VecDeque::new(),
            pc: 0,
            rel_base: 0,
        }
    }
    fn decode(&self) -> (Instr, usize) {
        use Instr::*;
        let mut pc = self.pc;
        let instr = self.mem[pc];
        pc += 1;
        // this closure will increment pc and shift mode.
        let mut mode = instr / 100;
        let pc_ref = &mut pc;
        let mut make_arg = move || {
            let val = self.mem[*pc_ref];
            *pc_ref += 1;
            let ret = match mode % 10 {
                0 => Arg::Pos(val as usize),
                1 => Arg::Imm(val),
                2 => Arg::Rel(val, self.rel_base),
                i => panic!("Unknown mode {}", i),
            };
            mode /= 10;
            ret
        };

        let instr = match instr % 100 {
            1 => Add(make_arg(), make_arg(), make_arg()),
            2 => Mul(make_arg(), make_arg(), make_arg()),
            3 => Input(make_arg()),
            4 => Output(make_arg()),
            5 => JumpIf(make_arg(), make_arg()),
            6 => JumpUnless(make_arg(), make_arg()),
            7 => LessThan(make_arg(), make_arg(), make_arg()),
            8 => Equals(make_arg(), make_arg(), make_arg()),
            9 => SetRelBase(make_arg()),
            99 => Halt,
            i => panic!("unknown opcode {} at pc {}", i, self.pc),
        };
        (instr, pc)
    }
    fn step(&mut self) -> State {
        let (instr, new_pc) = self.decode();
        if let Instr::Input(..) = instr {
            if self.input.is_empty() {
                return State::NeedsInput;
            }
        }
        self.pc = new_pc;
        instr.execute(self)
    }
}

use itertools::Itertools;
use std::collections::HashMap;

fn output_to_string(comp: &mut IntComp) -> String {
    comp.output
        .drain(..)
        .map(|c| std::char::from_u32(c as u32).unwrap())
        .collect::<String>()
}

fn run_part1(mem: Vec<isize>) {
    let mut comp = IntComp::new(mem.clone(), &[]);
    while let State::Run = comp.step() {}
    let map = output_to_string(&mut comp);
    println!("{}", &map);
    let mut scaffolds = HashMap::new();
    let mut max = (0isize, 0isize);
    for (y, line) in map.trim().lines().enumerate() {
        let y = y as isize;
        for (x, c) in line.chars().enumerate() {
            let x = x as isize;
            scaffolds.insert((x, y), c);
            max.0 = x;
        }
        max.1 = y;
    }
    let mut alignment_sum = 0;
    for (x, y) in (0..=max.0).cartesian_product(0..=max.1) {
        if [(x, y), (x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
            .iter()
            .all(|pos| scaffolds.get(pos).map(|&c| c == '#') == Some(true))
        {
            alignment_sum += x * y;
        }
    }
    println!("alignment sum: {}", alignment_sum);

    let start_pos = scaffolds.iter().find(|(_pos, val)| **val == '^').unwrap().0;
    let mut dirs = String::new();
    let mut bot = Bot {
        pos: *start_pos,
        dir: Dir::North,
    };
    loop {
        if let Some('#') = scaffolds.get(&bot.to_front()) {
            dirs += "F";
            bot.step();
        } else if let Some('#') = scaffolds.get(&bot.to_left()) {
            dirs += "L";
            bot.turn_left();
        } else if let Some('#') = scaffolds.get(&bot.to_right()) {
            dirs += "R";
            bot.turn_right();
        } else {
            break;
        }
    }
    println!(
        "bot ended at {},{} with {} directions {}",
        bot.pos.0,
        bot.pos.1,
        dirs.len(),
        dirs
    );
    let compressed = dbg!(compress(&dirs).pop().unwrap());

    comp.mem[0] = 2;
    comp.input.extend(compressed.chars().map(|c| c as isize));
    comp.input.extend(['n' as isize, '\n' as isize].iter());
    while let State::Run = comp.step() {}
    let mut answer = None;
    if let Some(c) = comp.output.back() {
        if *c > 256 {
            answer = Some(*c);
            comp.output.pop_back();
        }
    }
    println!("final output\n{}", output_to_string(&mut comp));
    println!("Answer: {:?}", answer);
}

// finds all possible compressions of `input`
fn compress(input: &str) -> Vec<String> {
    let mut result = Vec::new();
    let max1 = (1..input.len())
        .rev()
        .find(|len| encode(&input[0..*len]).len() <= 20)
        .unwrap();
    for split1 in (1..max1).rev() {
        if let Some(max2) = (1..(input.len() - split1))
            .rev()
            .find(|len| encode(&input[split1..(split1 + len)]).len() <= 20)
        {
            let (str1, mut rest) = input.split_at(split1);
            while rest.starts_with(str1) {
                rest = &rest[str1.len()..];
            }

            for (split2, split3) in (1..max2).rev().cartesian_product(1..input.len() / 2) {
                if split1 + split2 + split3 > input.len() {
                    continue;
                }
                let (str2, _rest) = rest.split_at(split2);
                if encode(str1).len() > 20 || encode(str2).len() > 20 {
                    continue;
                }

                let mut tasks = VecDeque::new();
                struct Coder<'a> {
                    pos: usize,
                    strs: Vec<&'a str>,
                    subs: Vec<u8>,
                }
                tasks.push_back(Coder {
                    pos: 0,
                    strs: vec![str1, str2],
                    subs: vec![],
                });

                while let Some(Coder {
                    mut pos,
                    mut strs,
                    mut subs,
                }) = tasks.pop_front()
                {
                    // encode words
                    if pos < input.len() {
                        let mut noncodable = true;
                        for (i, s) in strs.iter().enumerate() {
                            if input[pos..].starts_with(s) {
                                noncodable = false;
                                pos += s.len();
                                subs.push(i as u8);
                                if subs.len() <= 11 {
                                    tasks.push_front(Coder {
                                        pos,
                                        strs: strs.clone(),
                                        subs: subs.clone(),
                                    });
                                }
                            }
                        }
                        if noncodable {
                            // found a non-codable spot, try split3
                            if strs.len() < 3 && pos + split3 < input.len() {
                                let str3 = &input[pos..(pos + split3)];
                                if encode(str3).len() <= 20 && subs.len() <= 11 {
                                    strs.push(str3);
                                    tasks.push_front(Coder { pos, strs, subs });
                                    // new str will be picked up on next loop
                                    // println!("{} {} {}", str1, str2, str3);
                                }
                            }
                        }
                    } else {
                        // encoded all words.
                        use std::fmt::Write;
                        let mut output = String::new();
                        writeln!(
                            &mut output,
                            "{}",
                            subs.iter()
                                .map(|&i| std::char::from_u32('A' as u32 + i as u32).unwrap())
                                .join(",")
                        );
                        writeln!(&mut output, "{}", encode(strs[0]));
                        writeln!(&mut output, "{}", encode(strs[1]));
                        writeln!(&mut output, "{}", encode(strs[2]));
                        result.push(output);
                    }
                }
            }
        }
    }
    result
}

#[test]
fn test_compress() {
    let s: String = "R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2"
        .chars()
        .flat_map(|c| -> Box<dyn Iterator<Item = char>> {
            match c {
                'R' | 'L' => Box::new(std::iter::once(c)),
                ',' => Box::new(std::iter::empty()),
                i => Box::new(std::iter::repeat('F').take(i.to_digit(10).unwrap() as usize)),
            }
        })
        .collect();
    assert!(compress(&s).contains(&String::from(
        "A,B,C,B,A,C\nR,8,R,8\nR,4,R,4,R,8\nL,6,L,2\n"
    )));
}

fn encode(word: &str) -> String {
    let mut output = Vec::new();
    let mut effs = 0;
    for c in word.chars() {
        if c == 'F' {
            effs += 1
        } else {
            if effs > 0 {
                output.push(format!("{}", effs));
            }
            effs = 0;
            output.push(c.to_string());
        }
    }
    if effs > 0 {
        output.push(format!("{}", effs));
    }
    output.join(",")
}

enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn to_left(&self) -> Dir {
        match self {
            Dir::North => Dir::West,
            Dir::West => Dir::South,
            Dir::South => Dir::East,
            Dir::East => Dir::North,
        }
    }
    fn to_right(&self) -> Dir {
        self.to_left().to_left().to_left()
    }
}

struct Bot {
    pos: (isize, isize),
    dir: Dir,
}

impl Bot {
    fn to_left(&self) -> (isize, isize) {
        self.pos.to_dir(&self.dir.to_left())
    }
    fn to_right(&self) -> (isize, isize) {
        self.pos.to_dir(&self.dir.to_right())
    }
    fn to_front(&self) -> (isize, isize) {
        self.pos.to_dir(&self.dir)
    }
    fn turn_left(&mut self) {
        self.dir = self.dir.to_left()
    }
    fn turn_right(&mut self) {
        self.dir = self.dir.to_right()
    }
    fn step(&mut self) {
        self.pos = self.pos.to_dir(&self.dir)
    }
}

trait Pos {
    fn to_dir(&self, dir: &Dir) -> Self;
}

impl Pos for (isize, isize) {
    fn to_dir(&self, dir: &Dir) -> Self {
        match dir {
            Dir::North => (self.0, self.1 - 1),
            Dir::South => (self.0, self.1 + 1),
            Dir::West => (self.0 - 1, self.1),
            Dir::East => (self.0 + 1, self.1),
        }
    }
}

fn main() {
    let mut mem: Vec<isize> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    mem.resize(mem.len() + 10000, 0);
    run_part1(mem.clone());
}
