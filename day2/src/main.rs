fn main() {
    let mut mem: Vec<usize> = include_str!("input.txt")
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    mem.extend([0;3].iter());  // so I can always deref all operands

    dbg!(run(mem.clone(), 12, 2));

    for noun in 0..100 {
        for verb in 0..100 {
            if run(mem.clone(), noun, verb) == 19690720 {
                dbg!(100 * noun + verb);
            }
        }
    }
}

fn run(mut mem: Vec<usize>, noun: usize, verb: usize) -> usize {
    mem[1] = noun;
    mem[2] = verb;

    let mut pc = 0;
    while let [op, arg1, arg2, dest] = mem[pc..pc+4] {
        mem[dest] = match op {
            1 => mem[arg1] + mem[arg2],
            2 => mem[arg1] * mem[arg2],
            99 => break,
            _ => panic!("unknown operation {} at pc {}", op, pc),
        };
        pc += 4;
    }
    mem[0]
}
