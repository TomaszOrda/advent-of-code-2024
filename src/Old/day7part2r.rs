//a bit faster version suing stack, thanks to the post of gorbit99
fn concatenate(left:u64, right:u64) -> u64{
    format!("{left}{right}").parse::<u64>().unwrap()
}


fn is_possible(equation: Vec<u64>) -> u64{
    let mut stack = vec![equation];
    while let Some(eq) = stack.pop(){
        if eq.len()==2 {
            if eq[0] == eq[1]{
                return eq[0]
            }else{
                continue
            }
        }

        [|a,b| a+b, |a,b| a*b, concatenate].iter()
                                   .map(
                    |f| 
                    {
                        let mut case = eq[3..].to_vec();
                        case.insert(0, f(eq[1], eq[2]));
                        case.insert(0, eq[0]);
                        case
                    }).for_each(|e| stack.push(e));
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

    format!("{:?}",equations.into_iter().map(is_possible).sum::<u64>() )
}