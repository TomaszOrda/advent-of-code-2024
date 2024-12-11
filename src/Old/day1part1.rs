use std::iter::zip;
fn total_distance_between_lists(input: String) ->i32{
    let mut left = vec![];
    let mut right = vec![];

    input.split_whitespace()
         .map(|x| x.parse::<i32>().unwrap())
         .enumerate()
         .for_each(|(id, x)| if id%2 == 0 {left.push(x)} else {right.push(x)} );

    left.sort();
    right.sort();

    zip(left, right).map(|(l,r)| (l-r).abs() ).sum()
}
pub fn solution(input:String) -> String {

    format!("{}",total_distance_between_lists(input))
}