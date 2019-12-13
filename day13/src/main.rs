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

use std::collections::HashMap;

fn run_part1(mem: Vec<isize>) {
    let mut comp = IntComp::new(mem, &[]);
    while let State::Run = comp.step() {}

    let mut map = HashMap::new();
    while let (Some(x), Some(y), Some(tile_id)) = (
        comp.output.pop_front(),
        comp.output.pop_front(),
        comp.output.pop_front(),
    ) {
        if tile_id == 2 {
            // block type
            map.insert((x, y), tile_id);
        }
    }
    println!("Number of block tiles: {}", map.len());
}

fn run_part2(mut mem: Vec<isize>) {    
    mem[0] = 2; // per instructions
    let mut comp = IntComp::new(mem, &[]);
    let mut last_ball = 0;
    let mut last_paddle = 0;
    let mut max_y = 0;
    
    print!("{}{}", ansi_escapes::ClearScreen, ansi_escapes::CursorHide);

    loop {
        match comp.step() {
            State::Halt => break,
            State::Run => (),
            State::NeedsInput => {
                use std::cmp::Ordering::*;
                comp.input.push_back(match last_paddle.cmp(&last_ball) {
                    Greater => -1,
                    Less => 1,
                    Equal => 0,
                });
                // use input as a chance to flush and induce some delay.
                use std::io::Write;
                let _ = std::io::stdout().lock().flush();    
                std::thread::sleep(std::time::Duration::from_millis(1));
            },
        }
        while comp.output.len() >= 3 {
            let (x, y, tile_id) = (
                comp.output.pop_front().unwrap(),
                comp.output.pop_front().unwrap(),
                comp.output.pop_front().unwrap(),
            );
            if x == -1 && y == 0 {
                print!("{}SCORE: {}", ansi_escapes::CursorTo::AbsoluteXY(0,0), tile_id);
            } else {
                // it's a draw instruction
                let out = match tile_id {
                    0 => ' ', // is an empty tile. No game object appears in this tile.
                    1 => '█', // is a wall tile. Walls are indestructible barriers.
                    2 => '□', // is a block tile. Blocks can be broken by the ball.
                    3 => {
                        last_paddle = x;
                        '⬌' // is a horizontal paddle tile. The paddle is indestructible.
                    },
                    4 => {
                        last_ball = x;
                        '●' // is a ball tile. The ball moves diagonally and bounces off objects.
                    },
                    i => panic!("unknown tile type: {}", i),
                };
                print!("{}{}", ansi_escapes::CursorTo::AbsoluteXY((y+1) as u16, x as u16), out);
                max_y = max_y.max(y+1);
            }
        }
    }

    // for tidyness, move past the board.
    print!("{}", ansi_escapes::CursorTo::AbsoluteXY(max_y as u16 + 5, 0));
    print!("{}", ansi_escapes::CursorShow);
    
}

fn main() {
    let mut mem: Vec<isize> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    mem.resize(mem.len() + 10000, 0);
    run_part1(mem.clone());
    run_part2(mem.clone());
}
