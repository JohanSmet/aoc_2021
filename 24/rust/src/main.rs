use std::collections::HashSet;

/* Ok, this was a hard one.

   First attempt was a naive one. I just implemented the ALU and hoped it would finish in a
   decent timespan (at least for part 1). Yeah, not happening.

   For the second attempt I stared at the input program, noticed it kinda repeated 14 times
   and decided to 'decompile' it by hand to see if I could find a pattern. With the logic
   reimplemented in rust I noticed the stack-like operations on z. But I thought, maybe
   the rust version is fast enough to brute force it ... Nope, not on my machine.

   Ok, new approach needed. Before trying to analyze the program further to find a better
   algorithmic approach, I wanted to try if dynamic programming would be sufficient to
   find a solution. So another rewrite to a recursive implementation and cache when
   a certain digit and start stack-value (z) does not lead to solution.

   Now the solution is found in about two seconds ... job done.
*/

/*
type Reg = usize;

enum Op {
    Input(Reg),
    Add(Reg, Reg),
    AddImm(Reg, i64),
    Mul(Reg, Reg),
    MulImm(Reg, i64),
    Div(Reg, Reg),
    DivImm(Reg, i64),
    Mod(Reg, Reg),
    ModImm(Reg, i64),
    Eql(Reg, Reg),
    EqlImm(Reg, i64),
}

#[derive(Debug)]
struct Alu {
    regs: [i64; 4],
}

fn is_reg(reg: &str) -> bool {
    reg == "w" || reg == "x" || reg == "y" || reg == "z"
}

fn reg_from_string(reg: &str) -> Reg {
    match reg {
        "w" => 0,
        "x" => 1,
        "y" => 2,
        "z" => 3,
        _ => unreachable!()
    }
}

impl Op {
    pub fn from_string(instr: &str) -> Self {
        let parts : Vec<&str> = instr.split_whitespace().collect();

        match parts[0] {
            "inp" =>    Op::Input(reg_from_string(parts[1])),
            "add" => {
                if is_reg(parts[2]) {
                    Op::Add(reg_from_string(parts[1]), reg_from_string(parts[2]))
                } else {
                    Op::AddImm(reg_from_string(parts[1]), parts[2].parse::<i64>().unwrap())
                }
            },
            "mul" => {
                if is_reg(parts[2]) {
                    Op::Mul(reg_from_string(parts[1]), reg_from_string(parts[2]))
                } else {
                    Op::MulImm(reg_from_string(parts[1]), parts[2].parse::<i64>().unwrap())
                }
            },
            "div" => {
                if is_reg(parts[2]) {
                    Op::Div(reg_from_string(parts[1]), reg_from_string(parts[2]))
                } else {
                    Op::DivImm(reg_from_string(parts[1]), parts[2].parse::<i64>().unwrap())
                }
            },
            "mod" => {
                if is_reg(parts[2]) {
                    Op::Mod(reg_from_string(parts[1]), reg_from_string(parts[2]))
                } else {
                    Op::ModImm(reg_from_string(parts[1]), parts[2].parse::<i64>().unwrap())
                }
            },
            "eql" => {
                if is_reg(parts[2]) {
                    Op::Eql(reg_from_string(parts[1]), reg_from_string(parts[2]))
                } else {
                    Op::EqlImm(reg_from_string(parts[1]), parts[2].parse::<i64>().unwrap())
                }
            },

            _ => unreachable!()
        }
    }
}


impl Alu {
    pub fn new() -> Self {
        Alu { regs: [0; 4] }
    }

    pub fn execute_op(&mut self, op: &Op, input_iter: &mut std::slice::Iter<i32>) {
        match *op {
            Op::Input(reg) =>           self.regs[reg] = *input_iter.next().expect("input data") as i64,
            Op::Add(reg_a, reg_b) =>    self.regs[reg_a] += self.regs[reg_b],
            Op::AddImm(reg, imm) =>     self.regs[reg] += imm,
            Op::Mul(reg_a, reg_b) =>    self.regs[reg_a] *= self.regs[reg_b],
            Op::MulImm(reg, imm) =>     self.regs[reg] *= imm,
            Op::Div(reg_a, reg_b) =>    self.regs[reg_a] /= self.regs[reg_b],
            Op::DivImm(reg, imm) =>     self.regs[reg] /= imm,
            Op::Mod(reg_a, reg_b) =>    self.regs[reg_a] %= self.regs[reg_b],
            Op::ModImm(reg, imm) =>     self.regs[reg] %= imm,
            Op::Eql(reg_a, reg_b) =>    self.regs[reg_a] = if self.regs[reg_a] == self.regs[reg_b] {1} else {0},
            Op::EqlImm(reg_a, imm) =>   self.regs[reg_a] = if self.regs[reg_a] == imm {1} else {0},
        }
    }

    pub fn execute_program(&mut self, program: &[Op], input: &[i32]) {
        let mut input_iter = input.iter();

        for op in program {
            self.execute_op(op, &mut input_iter)
        }
    }
}
*/

fn execute_block(input: i64, constants: &(i64, i64, i64), z: i64) -> i64 {
    let x = (z % 26) + constants.1;
    let z = z / constants.0;
    if x != input {(z * 26) + input + constants.2} else {z}
}

fn find_serial(constants: &[(i64, i64, i64)], digits: &[i64], current: usize, z: i64, failures: &mut HashSet<(usize, i64)>) -> Option<i64> {

    if current >= constants.len() {
        // at the end of the serial number, check if it's valid
        return if z != 0 {None} else {Some(0)}
    }

    // check if we already tried this combination without success
    if failures.contains(&(current, z)) {
        return None;
    }

    // try all valid digits
    for &digit in digits {

        // execute the block for this digit
        let next_z = execute_block(digit, &constants[current], z);

        // recurse to the next block
        if let Some(serial) = find_serial(constants, digits, current + 1, next_z, failures) {
            return Some((digit * 10i64.pow(13 - current as u32)) + serial);
        }
    }

    // save the fact that we didn't find an answer for this combination
    failures.insert((current, z));
    None
}

fn main() {
    let program_str = include_str!("../../monad.txt").trim_end();

    // extract the constants from the monad code so it should work with different input
    //  - assuming the constants are the only differences between the monads
    let lines = program_str.lines().collect::<Vec<_>>();
    let constants = lines
                    .chunks(18)
                    .map(|chunk| {
                        (chunk[4][6..].parse::<i64>().unwrap(),
                         chunk[5][6..].parse::<i64>().unwrap(),
                         chunk[15][6..].parse::<i64>().unwrap())
                    }).collect::<Vec<_>>();


    let part1 = find_serial(&constants, &[9,8,7,6,5,4,3,2,1], 0, 0, &mut HashSet::new());
    println!("Part 1 = {}", part1.unwrap());

    let part2 = find_serial(&constants, &[1,2,3,4,5,6,7,8,9], 0, 0, &mut HashSet::new());
    println!("Part 2 = {}", part2.unwrap());
}
