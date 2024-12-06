use std::{env, fs::read_to_string};
use::std::iter::zip;

fn similarity_score_between_lists(left:&mut Vec<i32>, right:&mut Vec<i32>) ->i32{
    left.sort();
    right.sort();

    zip(left, right).map(|(l,r)| (*l - *r).abs() ).sum()
}

fn main() {
    let args = &mut env::args();
    args.next();
    let input_file : String = args.next().unwrap_or_else(|| format!("No input file provided"));
    
    let mut left = vec![];
    let mut right = vec![];
    read_to_string(input_file).unwrap().lines().for_each(|line| {
        let values: Vec<i32> = line.to_string().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        left.push(values[0]);
        right.push(values[1]);
    });

    println!("{}", 
        similarity_score_between_lists(&mut left, &mut right)
    );
}
