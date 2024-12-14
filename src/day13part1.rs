type F = fraction::Fraction; 
fn tokens(a: (u32, u32), b: (u32, u32), p: (u32, u32)) -> u32{
    let a = (F::from(a.0), F::from(a.1));
    let b = (F::from(b.0), F::from(b.1));
    let p = (F::from(p.0), F::from(p.1));

    //Never fires, if it did it would just add one simple case to the function; I am lazy, ill leave here as is.
    if (a.0 /b.0 - a.1/b.1).abs() == F::from(0.00){
        panic!("Non independent!")
    }

    let b_presses =  (p.0 - p.1*a.0/a.1)/(b.0 - a.0*b.1/a.1);
    let a_presses = (p.0 - b_presses *b.0)/a.0;
    if b_presses.denom().unwrap()==&1 && a_presses.denom().unwrap()==&1 && b_presses.numer().unwrap()<=&100 && a_presses.numer().unwrap()<=&100 {
        *b_presses.numer().unwrap() as u32 + *a_presses.numer().unwrap() as u32 * 3
    }else{
        0
    }
}

fn trim(word:&str) -> &str{
    word.trim_end_matches([':', ','])
        .trim_start_matches(['X', 'Y', '+', '='])
}

pub fn solution(input: String) -> String { 
    let claw_machines = input
        .lines()
        .filter(
            |line|
            line.len()>0)
        .map(
            |line| 
            line.split_whitespace()
                .map(|w| trim(w))
                .collect::<Vec<&str>>())
        .map(
            |words|
            match words[0]{
                "Button" => (words[1], (words[2].parse::<u32>().unwrap(), words[3].parse::<u32>().unwrap())),
                "Prize"  => ("P"     , (words[1].parse::<u32>().unwrap(), words[2].parse::<u32>().unwrap())),
                _        => panic!("Unexpected line!")
            })
        .collect::<Vec<(&str, (u32, u32))>>();
    format!("{:?}",claw_machines.chunks_exact(3).map(|machine| tokens(machine[0].1, machine[1].1, machine[2].1) ).sum::<u32>()) 
} 
