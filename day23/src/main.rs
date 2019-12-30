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

fn output_to_string(comp: &mut IntComp) -> String {
    comp.output
        .drain(..)
        .filter_map(|c| std::char::from_u32(c as u32).or_else(|| {
            println!("Large value {}", c);
            None
        }))
        .collect::<String>()
}

fn main() {
    let mut mem: Vec<isize> = include_str!("input")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    mem.resize(mem.len() + 1024, 0);
    run_part1(mem.clone());
    // run_part2(mem.clone());
}

use std::collections::HashMap;

fn run_part1(mem: Vec<isize>) {
    let mut comps : Vec<_> = (0..50).map(|addr| IntComp::new(mem.clone(), &[addr])).collect();
    let mut nat_pkt : Option<(isize,isize)> = None;
    let mut last_nat_pkt = nat_pkt;
    loop {
        if let Some((x,y)) = nat_pkt {
            if comps.iter().all(|comp| comp.input.len() == 1) {
                // everyone is idle
                comps[0].input.push_back(x);
                comps[0].input.push_back(y);
                if last_nat_pkt == nat_pkt {
                    println!("delivered {:?} twice in a row", nat_pkt);
                    return;
                }
                last_nat_pkt = nat_pkt;
            }
        }
        for comp_idx in 0..comps.len() {
            let comp = &mut comps[comp_idx];
            if !comp.input.is_empty() {
                while match comp.step() {
                    State::Run => true,
                    State::Halt => false,
                    State::NeedsInput => {
                        comp.input.push_back(-1);
                        false // stop for now.
                    },
                } {
                    // no body.
                }
                let messages : Vec<_> = comp.output.drain(..).collect();
                for message in messages[..].chunks(3) {
                    let dest = message[0];
                    let x = message[1];
                    let y = message[2];
                    //println!("{} -> {}: {},{}", comp_idx, dest, x, y);
                    if let Some(dest_comp) = comps.get_mut(dest as usize) {
                        dest_comp.input.push_back(x);
                        dest_comp.input.push_back(y);
                    } else {
                        if nat_pkt.is_none() {
                            println!("Received first traffic for NAT {}: {},{}", dest, x, y);
                        }
                        nat_pkt = Some((x,y));
                    }
                }
            }
        }    
    }
}


