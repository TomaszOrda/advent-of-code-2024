type F = fraction::Fraction; 

fn tokens(a: (u64, u64), b: (u64, u64), p: (u64, u64)) -> u64{
    let a = (F::from(a.0), F::from(a.1));
    let b = (F::from(b.0), F::from(b.1));
    let p = (F::from(p.0), F::from(p.1));

    //Never fires, if it did it would just add one simple case to the function; I am lazy, ill leave here as is.
    if (a.0 /b.0 - a.1/b.1).abs() == F::from(0.00){
        panic!("Non independent!")
    }

    let b_presses =  (p.0 - p.1*a.0/a.1)/(b.0 - a.0*b.1/a.1);
    let a_presses = (p.0 - b_presses *b.0)/a.0;
    if b_presses.denom().unwrap()==&1 && a_presses.denom().unwrap()==&1 {
        *b_presses.numer().unwrap() + *a_presses.numer().unwrap() * 3
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
            !line.is_empty())
        .map(
            |line| 
            line.split_whitespace()
                .map(trim)
                .collect::<Vec<&str>>())
        .map(
            |words|
            match words[0]{
                "Button" => (words[1], (words[2].parse::<u64>().unwrap(), words[3].parse::<u64>().unwrap())),
                "Prize"  => ("P"     , (words[1].parse::<u64>().unwrap() + 10000000000000, words[2].parse::<u64>().unwrap() + 10000000000000)),
                _        => panic!("Unexpected line!")
            })
        .collect::<Vec<(&str, (u64, u64))>>();
    format!("{:?}",claw_machines.chunks_exact(3).map(|machine| tokens(machine[0].1, machine[1].1, machine[2].1) ).sum::<u64>()) 

} 
