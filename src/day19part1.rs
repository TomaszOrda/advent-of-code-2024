fn is_possible(design: &str, towels: &[&str]) -> bool{
    if design.is_empty(){
        return true
    }

    towels
        .iter()
        .filter(
            |&&towel|
            design.starts_with(towel))
        .any(
            |towel|
            is_possible(&design[towel.len()..], towels))

}

fn is_reundant(towel_id: usize, towels:&[&str]) -> bool{
    is_possible(towels[towel_id], &towels[towel_id+1..])
}

pub fn solution(input: String) -> String { 
    let mut towels = input
            .lines()
            .next()
            .unwrap()
            .split(", ")
            .collect::<Vec<&str>>();
    towels.sort_by(|a,b| b.len().cmp(&a.len()));
    //by removing redundancy the recursion does not branch as much
    let towels = towels.iter().enumerate().filter(|towel| !is_reundant(towel.0, &towels)).map(|towel| *towel.1).collect::<Vec<&str>>();
    let designs = input
        .lines()
        .skip(2)
        .collect::<Vec<&str>>();

    
    format!("{:?}",designs.iter().filter(|design| is_possible(design, &towels)).count()) 
} 