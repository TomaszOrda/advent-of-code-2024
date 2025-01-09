//Recursion is rather slow. Maybe working the list from the end would be a bit faster.
//Realized i cannot prune. Also this concatenate pretty but slow.

fn concatenate(left:u64, right:u64) -> u64{
    let mut multiplier = 1;

    while multiplier <=right{
        multiplier *=10;
    }
    right + left*multiplier
    // 10u64.pow((right as f64).log10() as u32 + 1) as u64 *left + right
    // format!("{left}{right}").parse::<u64>().unwrap()
}

fn is_possible(equation: &[u64]) -> bool{
    if equation.len() == 2{
        return equation[0] == equation[1]
    }
    [|a,b| a+b, |a,b| a*b, concatenate].iter()
                                    .map(
                    |f| 
                    {   
                        let mut case = equation[3..].to_vec();
                        case.insert(0, f(equation[1], equation[2]));
                        case.insert(0, equation[0]);
                        is_possible(&case)
                    }).any(|b| b)

}

pub fn solution(input: String) -> String {
    let equations: Vec<Vec<u64>> = input
                                    .lines()
                                    .map(
                                        |line| 
                                        line.split_whitespace().map(|s| s.trim_end_matches(':').parse::<u64>().unwrap()).collect::<Vec<u64>>())
                                    .collect();

    format!("{:?}",equations.iter().map(|e| if is_possible(e) {e[0]} else {0}).sum::<u64>() )
}