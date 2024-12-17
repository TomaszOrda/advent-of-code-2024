//It is not the nicest code it could be. But I am afraid that in general it would be an extremel hard task.
//By running the code one can notice that the output changes in a very orderly way
//nth output value appears in constant blocks of length 8^(n+1) iteartions,
//Length of the output also changes in a predictable way (as can be deduced from the code) nth iteration produces 1 + log_8 n output values (just as if the first value was empty)
//Thus starting with 8^15 (smallest possible A value for which program outputs enough values) we skip 8^15 values untill the last value matches
//We repeat that for every output value

//My guess is that this task is about making educated guess about the processes.
//And this might be just the place to do so. Escpecially considering the story.
//We can make the guess and then check if it is true (run the code and check the return value)

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
    //bitwise shift works faster of course. It started making a differene since part 2
    let new_val = reg.a >> combo(reg, arg); // / 2_u64.pow(combo(reg, arg));
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
fn out(reg: &Register, arg: u8) -> u64 {
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

    //Here powers are not that expensive, we will use pow instead of bitshift
    let mut guess = 8_u64.pow(instructions.len() as u32 - 1); //First guess that has long enough output. Refer to the paragraph at the beginning.
    let mut pointer = instructions.len() - 1;
    let mut skip_length = 8_u64.pow(instructions.len() as u32 - 1); //Refer to the same paragraph.
                                                                    // I could do this with recursion, however I do not think it would simply anything
    loop {
        while run_code(&mut Register::new_guess_a(&registers, guess), &instructions)
            .split(",")
            .map(|c| c.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()[pointer]
            != instructions[pointer]
        {
            guess += skip_length;
        }
        skip_length /= 8;
        if pointer == 0 {
            break;
        } else {
            pointer -= 1;
        }
    }
    if run_code(&mut Register::new_guess_a(&registers, guess), &instructions)
        .split(",")
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<u8>>()
        == instructions
    {
        format!("{}", guess)
    } else {
        "Failure".to_string()
    }
}
