#[derive(Debug, Clone)]
struct Register {
    a: u64,
    b: u64,
    c: u64,
}
impl Register {
    fn new(v: Vec<u64>) -> Self {
        Self {
            a: v[0],
            b: v[1],
            c: v[2],
        }
    }
    fn new_guess_a(broken_register: &Register, a_value: u64) -> Self {
        Self {
            a: a_value,
            b: broken_register.b,
            c: broken_register.c,
        }
    }
}

fn combo(reg: &Register, operand: u8) -> u64 {
    match operand {
        literal if (0..=3).contains(&operand) => literal as u64,
        4 => reg.a,
        5 => reg.b,
        6 => reg.c,
        7 => panic!("7 reserved — thus not an operand!"),
        _ => panic!("Invalid operand"),
    }
}

fn adv(reg: &mut Register, arg: u8) {
    let new_val = reg.a >> combo(reg, arg) as u32;
    reg.a = new_val;
}
fn bxl(reg: &mut Register, arg: u8) {
    let new_val = reg.b ^ arg as u64;
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
fn out(reg: &Register, arg: u8) -> u8 {
    (combo(reg, arg) % 8) as u8
}
fn bdv(reg: &mut Register, arg: u8) {
    let new_val = reg.a >> combo(reg, arg);
    reg.b = new_val;
}
fn cdv(reg: &mut Register, arg: u8) {
    let new_val = reg.a >> combo(reg, arg);
    reg.c = new_val;
}

fn runs_exact(reg: &mut Register, instructions: &[u8]) -> bool {
    // let mut output = vec![];
    let mut output_pointer = 0;
    let output_size = instructions.len();
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
            5 => {
                if output_pointer >= output_size
                    || instructions[output_pointer] != out(reg, instructions[pointer + 1])
                {
                    return false;
                } else {
                    output_pointer += 1;

                    if output_pointer > output_size {
                        return false;
                    }
                }
            }
            6 => bdv(reg, instructions[pointer + 1]),
            7 => cdv(reg, instructions[pointer + 1]),
            _ => panic!("Unknown instruction!"),
        }
        if instruction != 3 {
            pointer += 2;
        }
    }
    output_pointer == output_size
}

pub fn solution(input: String) -> String {
    let registers = Register::new(
        input
            .lines()
            .take(3)
            .map(|line| {
                line.split_whitespace()
                    .last()
                    .unwrap()
                    .parse::<u64>()
                    .unwrap()
            })
            .collect::<Vec<u64>>(),
    );
    let instructions = input
        .lines()
        .last()
        .unwrap()
        .trim_start_matches("Program: ")
        .split(',')
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();
    //Naive approach runs well enough on test. Not well enough for task 2.
    for a_value in 0..u32::MAX as u64 {
        if runs_exact(
            &mut Register::new_guess_a(&registers, a_value),
            &instructions,
        ) {
            return format!("{}", a_value);
        }
    }

    "Not found".to_string()
}
