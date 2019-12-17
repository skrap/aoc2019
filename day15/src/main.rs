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
    let mut tasks = VecDeque::new();
    let comp = IntComp::new(mem.clone(), &[]);
    tasks.push_back((comp, (0_isize, 0_isize)));
    let mut bests = HashMap::new();
    bests.insert((0, 0), 0);
    let mut solution_pos = None;
    while let Some((last_comp, pos)) = tasks.pop_front() {
        for dir in &[1, 2, 3, 4] {
            let mut comp = last_comp.clone();
            comp.input.push_back(*dir);
            while let State::Run = comp.step() {}
            let new_pos = match dir {
                // north (1), south (2), west (3), and east (4)
                1 => (pos.0, pos.1 - 1),
                2 => (pos.0, pos.1 + 1),
                3 => (pos.0 - 1, pos.1),
                4 => (pos.0 + 1, pos.1),
                x => panic!("unknown dir {}", x),
            };
            match comp.output.back() {
                Some(&i) if i == 1 || i == 2 => {
                    // only try if it's still shorter than our best solution.
                    if let Some(moves) = bests.get(&new_pos) {
                        if *moves <= comp.output.len() {
                            continue;
                        }
                    }
                    bests.insert(new_pos, comp.output.len());
                    if i == 1 {
                        tasks.push_front((comp, new_pos));
                    } else {
                        println!("solution at {:?} dist {}", new_pos, comp.output.len());
                        solution_pos = Some(new_pos);
                    }
                }
                Some(0) => {
                    // println!("hit a wall. stop.");
                }
                i => panic!("unknown output {:?}", i),
            }
        }
    }

    let yminmax = bests.keys().map(|(_, y)| y).minmax().into_option().unwrap();
    let xminmax = bests.keys().map(|(x, _)| x).minmax().into_option().unwrap();

    for y in *yminmax.0..=*yminmax.1 {
        for x in *xminmax.0..=*xminmax.1 {
            print!(
                "{}",
                if bests.contains_key(&(x, y)) {
                    if Some((x, y)) == solution_pos {
                        "X"
                    } else {
                        "*"
                    }
                } else {
                    " "
                }
            );
        }
        println!();
    }
    println!(
        "solution {:?} at dist {}",
        solution_pos.unwrap(),
        bests.get(&solution_pos.unwrap()).unwrap()
    );

    run_part2(solution_pos.unwrap(), bests);
}

fn run_part2(start_pos: (isize, isize), valid_spots: HashMap<(isize, isize), usize>) {
    let mut bests = HashMap::new();
    bests.insert(start_pos, 0);
    let mut tasks = VecDeque::new();
    tasks.push_back((start_pos, 0));
    while let Some((pos, dist)) = tasks.pop_front() {
        enum Dir {
            Up,
            Down,
            Left,
            Right,
        }
        use Dir::*;
        for i in [Up, Down, Left, Right].iter() {
            let mut new_pos = pos;
            match i {
                Up => new_pos.1 -= 1,
                Down => new_pos.1 += 1,
                Left => new_pos.0 -= 1,
                Right => new_pos.0 += 1,
            }
            let new_dist = dist + 1;
            if valid_spots.contains_key(&new_pos) && *bests.get(&new_pos).unwrap_or(&(new_dist+1)) > new_dist {
                bests.insert(new_pos, new_dist);
                tasks.push_back((new_pos, new_dist));
            }
        }
    }
    println!("Longest time to get O2: {} minutes", bests.values().max().unwrap_or(&0));
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
