use std::collections::VecDeque;

struct IntComp {
    mem: Vec<i32>,
    input: VecDeque<i32>,
    output: VecDeque<i32>,
    pc: usize,
}

#[derive(Debug)]
enum State {
    Run,
    NeedsInput,
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
            Halt => (),
        };

        if let Halt = self {
            State::Halt
        } else {
            State::Run
        }
    }
}

fn decode(mem: &[i32], mut pc: usize) -> (Instr, usize) {
    use Instr::*;
    let start_pc = pc;
    let instr = mem[pc];
    pc += 1;
    let mut mode = instr / 100;
    let pc_ref = &mut pc;
    let mut make_arg = move || {
        let val = mem[*pc_ref];
        *pc_ref += 1;
        let ret = match mode % 10 {
            0 => Arg::Pos(val as usize),
            1 => Arg::Imm(val),
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
        99 => Halt,
        i => panic!("unknown opcode {} at pc {}", i, start_pc),
    };
    (instr, pc)
}

impl IntComp {
    fn new(mem: Vec<i32>, input: &[i32]) -> IntComp {
        IntComp {
            mem,
            input: input.iter().cloned().collect(),
            output: VecDeque::new(),
            pc: 0,
        }
    }

    fn step(&mut self) -> State {
        let (instr, new_pc) = decode(&self.mem, self.pc);
        if let Instr::Input(..) = instr {
            if self.input.is_empty() {
                return State::NeedsInput;
            }
        }
        self.pc = new_pc;
        instr.execute(self)
    }

    fn run_day_7(&mut self, input: i32) -> (i32, State) {
        self.input.push_back(input);
        let state = loop {
            match self.step() {
                State::Run => continue,
                i => break i,
            }
        };
        assert_eq!(self.output.len(), 1);
        (self.output.pop_back().unwrap(), state)
    }
}

fn permutations(input: Vec<i32>) -> Vec<Vec<i32>> {
    let mut picks = VecDeque::new();
    picks.push_back((vec![], input));
    let mut perms = Vec::new();
    while let Some((prefix, rest)) = picks.pop_front() {
        for (i, val) in rest.iter().enumerate() {
            let mut new_prefix = prefix.clone();
            new_prefix.push(*val);
            let mut new_rest = rest.clone();
            new_rest.swap_remove(i);
            picks.push_back((new_prefix, new_rest));
        }
        if rest.len() == 0 {
            perms.push(prefix);
        }
    }
    perms
}

fn main() {
    let mem: Vec<i32> = include_str!("input.txt")
        //"3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let perms = permutations(vec![4, 3, 2, 1, 0]);

    dbg!(perms.len());

    let result = perms
        .iter()
        .map(|phases| {
            let amp_a = IntComp::new(mem.clone(), &[phases[0]]).run_day_7(0).0;
            let amp_b = IntComp::new(mem.clone(), &[phases[1]]).run_day_7(amp_a).0;
            let amp_c = IntComp::new(mem.clone(), &[phases[2]]).run_day_7(amp_b).0;
            let amp_d = IntComp::new(mem.clone(), &[phases[3]]).run_day_7(amp_c).0;
            IntComp::new(mem.clone(), &[phases[4]]).run_day_7(amp_d).0
        })
        .max();
    println!("Part 1 best thrust: {:?}", result);

    let perms = permutations(vec![5, 6, 7, 8, 9]);
    let mut max_thrust = 0;
    for perm in &perms {
        let mut comps = [
            IntComp::new(mem.clone(), &[perm[0]]),
            IntComp::new(mem.clone(), &[perm[1]]),
            IntComp::new(mem.clone(), &[perm[2]]),
            IntComp::new(mem.clone(), &[perm[3]]),
            IntComp::new(mem.clone(), &[perm[4]]),
        ];
        let mut amp_a_input = 0;
        let thrust = loop {
            let amp_a = comps[0].run_day_7(amp_a_input).0;
            let amp_b = comps[1].run_day_7(amp_a).0;
            let amp_c = comps[2].run_day_7(amp_b).0;
            let amp_d = comps[3].run_day_7(amp_c).0;
            let (amp_e, state) = comps[4].run_day_7(amp_d);
            if let State::Halt = state {
                break amp_e;
            } else {
                amp_a_input = amp_e;
            }
        };
        max_thrust = max_thrust.max(thrust);
    }
    println!("Max thrust part 2: {}", max_thrust);
}
