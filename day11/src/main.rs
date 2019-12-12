use std::collections::VecDeque;

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

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

use std::collections::HashMap;

fn main() {
    let mut mem: Vec<isize> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    mem.resize(mem.len() + 10000, 0);
    
    
    let comp = IntComp::new(mem.clone(), &[]);
    let mut map : HashMap<(isize,isize), isize> = HashMap::new();
    run_robot(comp, &mut map);

    println!("part 1 output: {:?}", map.len());

    let comp = IntComp::new(mem.clone(), &[]);
    let mut map : HashMap<(isize,isize), isize> = HashMap::new();
    map.insert((0,0), 1);
    run_robot(comp, &mut map);
    let mut min = (0,0);
    let mut max = (0,0);
    for (pos, _val) in map.iter() {
        min.0 = min.0.min(pos.0);
        min.1 = min.1.min(pos.1);
        max.0 = max.0.max(pos.0);
        max.1 = max.1.max(pos.1);
    }
    for y in min.1..=max.1 {
        for x in min.0..=max.0 {
            if let Some(1) = map.get(&(x,y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn run_robot(mut comp: IntComp, map: &mut HashMap<(isize,isize), isize>) {
    let mut pos = (0isize, 0isize);
    let mut dir = Dir::Up;
        
    loop {
        match comp.step() {
            State::Run => {
                if comp.output.len() == 2 {
                    let color = comp.output.pop_front().unwrap();
                    let turn = comp.output.pop_front().unwrap();
                    map.insert(pos, color);
                    dir = match (dir,turn) {
                        (Dir::Up, 0) => Dir::Left,
                        (Dir::Left, 0) => Dir::Down,
                        (Dir::Down, 0) => Dir::Right,
                        (Dir::Right, 0) => Dir::Up,

                        (Dir::Up, 1) => Dir::Right,
                        (Dir::Right, 1) => Dir::Down,
                        (Dir::Down, 1) => Dir::Left,
                        (Dir::Left, 1) => Dir::Up,
                        
                        (_, i) => panic!("invalid turn direction {}", i),
                    };
                    match dir {
                        Dir::Up => pos.1 -= 1,
                        Dir::Down => pos.1 += 1,
                        Dir::Left => pos.0 -= 1,
                        Dir::Right => pos.0 += 1,
                    }
                }
            }
            State::NeedsInput => {
                comp.input.push_back(*map.get(&pos).unwrap_or(&0));
            }
            State::Halt => {
                break;
            }
        }
    }
}
