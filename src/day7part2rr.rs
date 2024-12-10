//a bit faster version suing stack, thanks to the post of gorbit99
//Finally dropped the time to under a second
//With new concatenate it runs below 37ms

fn concatenate(left:u64, right:u64) -> u64{
    let mut multiplier = 1;

    while multiplier <=right{
        multiplier *=10;
    }
    right + left*multiplier
    // 10u64.pow((right as f64).log10() as u32 + 1) as u64 *left + right
    // format!("{left}{right}").parse::<u64>().unwrap()
}

fn calibration_value(equation: Vec<u64>) -> u64{
    let mut stack = vec![(2,equation[1])];
    let max_depth = equation.len();
    while let Some(state) = stack.pop(){
        
        if state.1 > equation[0]{
            continue
        }
        if state.0 >= max_depth {
            if equation[0] == state.1{
                return equation[0]
            }else{
                continue
            }
        }
        stack.push( (state.0+1, state.1 + equation[state.0])                        );
        stack.push( (state.0+1, state.1 * equation[state.0])                        );
        stack.push( (state.0+1, concatenate(state.1, equation[state.0])));
    }
    0
}

pub fn solution(input: String) -> String {
    let equations: Vec<Vec<u64>> = input
                                    .lines()
                                    .map(
                                        |line| 
                                        line.split_whitespace().map(|s| s.trim_end_matches(':').parse::<u64>().unwrap()).collect::<Vec<u64>>())
                                    .collect();

    format!("{:?}",equations.into_iter().map(calibration_value).sum::<u64>())
}