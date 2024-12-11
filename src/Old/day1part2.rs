use std::collections::HashMap;
fn similarity_score_between_lists(input: String) ->i32{
    let mut left = vec![];
    let mut right = vec![];

    input.split_whitespace()
         .map(|x| x.parse::<i32>().unwrap())
         .enumerate()
         .for_each(|(id, x)| if id%2 == 0 {left.push(x)} else {right.push(x)} );

    let mut frequency: HashMap<i32, i32> = HashMap::new();

    right.iter().for_each(|&x| {frequency.entry(x).and_modify(|e| *e+=1).or_insert(1);});
    left.iter().map(|x| x*frequency.get(x).unwrap_or(&0)).sum()
}
pub fn solution(input:String) -> String {
    format!("{}",similarity_score_between_lists(input))
}
