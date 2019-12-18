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

fn run_part1(mem: Vec<isize>) {
    let mut comp = IntComp::new(mem.clone(), &[]);
    while let State::Run = comp.step() {}
    let map = comp
        .output
        .drain(..)
        .map(|c| std::char::from_u32(c as u32).unwrap())
        .collect::<String>();
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

    let start_pos = scaffolds.iter().find(|(_pos,val)| **val == '^').unwrap().0;
    let mut dirs = String::new();
    let mut bot = Bot { pos: *start_pos, dir: Dir::North };
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
    println!("bot ended at {},{} with {} directions {}", bot.pos.0, bot.pos.1, dirs.len(), dirs);
    let dirs : Vec<_> = dirs.bytes().collect();
    let compressed = compress(&dirs);
}

fn compress(input: &[u8]) {
    let big_l = loop {
        for 
    };
}

// LFFFFFFFFRFFFFFFFFFFFFLFFFFFFFFLFFFFFFFFRFFFFFFFFFFFFLFFFFFFFFRFFFFFFFFFFRFFFFFFFFFFLFFFFFFFFRFFFFFFFFFFFFRFFFFFFFFFFFFRFFFFFFFFFFLFFFFFFFFFFRFFFFFFFFFFRFFFFFFFFFFLFFFFFFFFRFFFFFFFFFFFFRFFFFFFFFFFFFRFFFFFFFFFFLFFFFFFFFFFRFFFFFFFFFFRFFFFFFFFFFLFFFFFFFFLFFFFFFFFRFFFFFFFFFFFFLFFFFFFFFRFFFFFFFFFFFFRFFFFFFFFFFFFRFFFFFFFFFFLFFFFFFFFFFLFFFFFFFFRFFFFFFFFFFFFLFFFFFFFF

enum Dir {
    North,
    South,
    East,
    West
}

impl Dir {
    fn to_left(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
    fn to_right(&self) -> Dir {
        self.to_left().to_left().to_left()
    }
}

struct Bot {
    pos: (isize,isize),
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

impl Pos for (isize,isize) {
    fn to_dir(&self, dir: &Dir) -> Self {
        match dir {
            Dir::North => (self.0,self.1-1),
            Dir::South => (self.0,self.1+1),
            Dir::West => (self.0-1,self.1),
            Dir::East => (self.0+1,self.1),
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
