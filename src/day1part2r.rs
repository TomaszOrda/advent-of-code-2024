use std::collections::HashMap;

fn similarity_score_between_lists(left:Vec<i32>, right:Vec<i32>) ->i32{
    let mut frequency: HashMap<i32, i32> = HashMap::new();

    right.iter().for_each(|&x| {frequency.entry(x).and_modify(|e| *e+=1).or_insert(1);});
    left.iter().map(|x| x*frequency.get(x).unwrap_or(&0)).sum()
}

pub fn solution(input: String) -> String {
    let mut left = vec![];
    let mut right = vec![];
    input.lines().for_each(|line| {
        let values: Vec<i32> = line.to_string().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        left.push(values[0]);
        right.push(values[1]);
    });

    format!("{}", 
        similarity_score_between_lists(left, right)
    )
}
