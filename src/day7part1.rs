


fn is_possible(equation: &[u64]) -> bool{
    if equation.len() == 2{
        return equation[0] == equation[1]
    }
    let mut addition_case = equation[3..].to_vec();
    addition_case.insert(0, equation[1] + equation[2]);
    addition_case.insert(0, equation[0]);
    let mut multiplication_case = equation[3..].to_vec();
    multiplication_case.insert(0, equation[1] * equation[2]);
    multiplication_case.insert(0, equation[0]);
    is_possible(&addition_case) || is_possible(&multiplication_case)

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