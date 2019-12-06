use std::collections::VecDeque;

struct IntComp {
    mem: Vec<i32>,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    pc: usize,
}

enum State {
    Run,
    Halt,
}

enum Arg {
    Imm(i32),
    Pos(usize),
}

impl Arg {
    fn load(&self, mem: &[i32]) -> i32 {
        match self {
            Arg::Imm(val) => *val,
            Arg::Pos(val) => mem[*val as usize],
        }
    }

    fn store(&self, val: i32, mem: &mut [i32]) {
        if let Arg::Pos(pos) = self {
            mem[*pos] = val;
        } else {
            panic!("Can't store to an immediate.");
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
            Input(arg) => arg.store(comp.input.pop_back().unwrap(), &mut comp.mem),
            Output(arg) => comp.output.push_front(arg.load(&comp.mem)),
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
            Halt => (),
        };

        if let Halt = self {
            State::Halt
        } else {
            State::Run
        }
    }
}

fn decode(mem: &[i32], pc: &mut usize) -> Instr {
    use Instr::*;
    let start_pc = *pc;
    let instr = mem[*pc];
    *pc += 1;
    let mut mode = instr / 100;
    let mut make_arg = move || {
        let val = mem[*pc];
        *pc += 1;
        let ret = match mode % 10 {
            0 => Arg::Pos(val as usize),
            1 => Arg::Imm(val),
            i => panic!("Unknown mode {}", i),
        };
        mode /= 10;
        ret
    };
    match instr % 100 {
        1 => Add(make_arg(), make_arg(), make_arg()),
        2 => Mul(make_arg(), make_arg(), make_arg()),
        3 => Input(make_arg()),
        4 => Output(make_arg()),
        5 => JumpIf(make_arg(), make_arg()),
        6 => JumpUnless(make_arg(), make_arg()),
        7 => LessThan(make_arg(), make_arg(), make_arg()),
        8 => Equals(make_arg(), make_arg(), make_arg()),
        99 => Halt,
        i => panic!("unknown opcode {} at pc {}", i, start_pc),
    }
}

impl IntComp {
    fn step(&mut self) -> State {
        let instr = decode(&self.mem, &mut self.pc);
        instr.execute(self)
    }
}

fn main() {
    println!("Running input 1");
    let mem: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut comp = IntComp {
        mem,
        input: VecDeque::new(),
        output: VecDeque::new(),
        pc: 0,
    };
    comp.input.push_back(1);
    while let State::Run = comp.step() {}
    dbg!(comp.output);

    println!("Running input 5");
    let mem: Vec<i32> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut comp = IntComp {
        mem,
        input: VecDeque::new(),
        output: VecDeque::new(),
        pc: 0,
    };
    comp.input.push_back(5);
    while let State::Run = comp.step() {}
    dbg!(comp.output);
}
