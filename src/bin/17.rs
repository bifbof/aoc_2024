// This one was soo much fun
// I overengineered the first part completely
// could have translated my program by hand 100x faster
// but writing a small interpreter was quite fun.
// and then finding the solution of the second part *chef kiss*
use itertools::Itertools;

type Program = Vec<u8>;
type RegId = usize;

#[derive(Debug)]
struct State {
    registers: [usize; 3],
    ip: usize,
    output: Vec<u8>,
}

impl State {
    fn new(registers: [usize; 3]) -> State {
        State {
            registers,
            ip: 0,
            output: Vec::new(),
        }
    }
}
#[derive(Debug)]
enum Operand {
    Literal(u8),
    Register(RegId),
    Invalid,
}

#[derive(Debug)]
enum Instruction {
    Div(RegId),
    Xor(RegId),
    Str(RegId),
    Jnz,
    Out,
}

fn main() {
    let (mut s, p) = parse();
    run(&mut s, &p);
    println!("{}", s.output.iter().map(|v| v.to_string()).join(","));
    let sols = search(&p);
    println!("{}", sols.iter().min().unwrap_or(&0))
}

fn run(s: &mut State, p: &Program) {
    while let Some((instr, oprnd)) = fetch(s, p) {
        exec(instr, oprnd, s);
    }
}

fn num_octal_digits(num: usize) -> Option<usize> {
    if num == 0 {
        None
    } else {
        Some((num.ilog2() / 3).try_into().unwrap())
    }
}

// how does this search work?
// the program works a bit like this
// B = A % 8
// ...
// A = A / 8
// JNZ to start
// so the last output is only dependent on the last digit of A
// we can find possible solutions for this
// and then repeat it for the next digit of A recursively
fn search(p: &Program) -> Vec<usize> {
    let mut stack = vec![0];
    let mut solutions = Vec::new();

    while let Some(num) = stack.pop() {
        if num_octal_digits(num).unwrap_or(0) == p.len() - 1 {
            solutions.push(num);
            continue;
        }
        let num = num * 8;
        for i in 0..8 {
            let new = num + i;
            let Some(len) = num_octal_digits(new) else {
                continue;
            };
            let mut s = State::new([new, 0, 0]);
            run(&mut s, p);
            if s.output[0] == p[p.len() - len - 1] {
                stack.push(new);
            }
        }
    }
    solutions
}

fn parse() -> (State, Program) {
    let data = std::fs::read_to_string("input/17.txt").unwrap();
    let (regs, program) = data.rsplit_once("\n\n").unwrap();
    let (_, program) = program.split_once(": ").unwrap();
    let program = program
        .trim()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let regs: Vec<usize> = regs
        .split("\n")
        .map(|line| {
            let (_, num) = line.split_once(": ").unwrap();
            num.parse().unwrap()
        })
        .collect();
    let state = State::new(regs.try_into().unwrap());
    (state, program)
}

fn fetch(s: &State, p: &Program) -> Option<(Instruction, Operand)> {
    use Instruction::*;
    use Operand::*;
    let instr = *p.get(s.ip)?;
    let oprnd = *p.get(s.ip + 1)?;
    let (instr, iscombo) = match instr {
        0 => (Div(0), true),
        1 => (Xor(1), false),
        2 => (Str(1), true),
        3 => (Jnz, false),
        4 => return Some((Xor(1), Register(2))),
        5 => (Out, true),
        6 => (Div(1), true),
        7 => (Div(2), true),
        _ => unreachable!(),
    };

    let oprnd = if iscombo {
        match oprnd {
            x if x <= 3 => Literal(x),
            4 => Register(0),
            5 => Register(1),
            6 => Register(2),
            _ => Invalid,
        }
    } else {
        Literal(oprnd)
    };

    Some((instr, oprnd))
}

fn get_value(s: &State, oprnd: Operand) -> usize {
    match oprnd {
        Operand::Literal(val) => val.into(),
        Operand::Register(rid) => s.registers[rid],
        Operand::Invalid => panic!("Cannot get value from invalid"),
    }
}

fn exec(instr: Instruction, oprnd: Operand, s: &mut State) {
    use Instruction::*;
    // maybe split up more in getting value, then storing value
    let val = match instr {
        Div(_) => s.registers[0] / (1 << get_value(s, oprnd)),
        Jnz => get_value(s, oprnd),
        Xor(dst) => s.registers[dst] ^ get_value(s, oprnd),
        Str(_) | Out => get_value(s, oprnd) % 8,
    };
    match instr {
        Div(dst) | Str(dst) | Xor(dst) => {
            s.registers[dst] = val;
            s.ip += 2;
        }
        Out => {
            s.output.push(val.to_le_bytes()[0]);
            s.ip += 2;
        }
        Jnz => {
            if s.registers[0] != 0 {
                s.ip = val
            } else {
                s.ip += 2
            }
        }
    }
}
