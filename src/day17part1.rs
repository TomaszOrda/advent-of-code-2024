#[derive(Debug)]
struct Register {
    a: u32,
    b: u32,
    c: u32,
}
impl Register {
    fn new(v: Vec<u32>) -> Self {
        Self {
            a: v[0],
            b: v[1],
            c: v[2],
        }
    }
}

fn combo(reg: &Register, operand: u8) -> u32 {
    match operand {
        literal if (0..=3).contains(&operand) => literal as u32,
        4 => reg.a,
        5 => reg.b,
        6 => reg.c,
        7 => panic!("7 reserved — thus not an operand!"),
        _ => panic!("Invalid operand"),
    }
}

fn adv(reg: &mut Register, arg: u8) {
    //bitwise shift works faster of course
    let new_val = reg.a >> combo(reg, arg); // / 2_u32.pow(combo(reg, arg));
    reg.a = new_val;
}
fn bxl(reg: &mut Register, arg: u8) {
    let new_val = reg.b ^ arg as u32;
    reg.b = new_val;
}
fn bst(reg: &mut Register, arg: u8) {
    let new_val = combo(reg, arg) % 8;
    reg.b = new_val;
}
fn jnz(reg: &Register) -> bool {
    reg.a != 0
}
fn bxc(reg: &mut Register, _arg: u8) {
    let new_val = reg.b ^ reg.c;
    reg.b = new_val;
}
fn out(reg: &Register, arg: u8) -> u32 {
    combo(reg, arg) % 8
}
fn bdv(reg: &mut Register, arg: u8) {
    let new_val = reg.a >> combo(reg, arg);
    reg.b = new_val;
}
fn cdv(reg: &mut Register, arg: u8) {
    let new_val = reg.a >> combo(reg, arg);
    reg.c = new_val;
}

fn run_code(reg: &mut Register, instructions: &[u8]) -> String {
    let mut output = vec![];
    let mut pointer: usize = 0;
    while pointer < instructions.len() {
        let instruction = instructions[pointer];
        match instruction {
            0 => adv(reg, instructions[pointer + 1]),
            1 => bxl(reg, instructions[pointer + 1]),
            2 => bst(reg, instructions[pointer + 1]),
            3 => {
                if jnz(reg) {
                    pointer = instructions[pointer + 1] as usize
                } else {
                    pointer += 2
                }
            }
            4 => bxc(reg, instructions[pointer + 1]),
            5 => output.push(out(reg, instructions[pointer + 1])),
            6 => bdv(reg, instructions[pointer + 1]),
            7 => cdv(reg, instructions[pointer + 1]),
            _ => panic!("Unknown instruction!"),
        }
        if instruction != 3 {
            pointer += 2;
        }
    }
    output
        .iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

pub fn solution(input: String) -> String {
    let mut registers = Register::new(
        input
            .lines()
            .take(3)
            .map(|line| {
                line.split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap()
            })
            .collect::<Vec<u32>>(),
    );
    let instructions = input
        .lines()
        .last()
        .unwrap()
        .trim_start_matches("Program: ")
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    run_code(&mut registers, &instructions)
}
