fn similarity_score_between_lists(left:&mut Vec<i32>, right:&mut Vec<i32>) ->i32{
    left.sort();
    right.sort();

    std::iter::zip(left, right).map(|(l,r)| (*l - *r).abs() ).sum()
}

pub fn solution(input:String) -> String{

    let mut left = vec![];
    let mut right = vec![];
    input.lines().for_each(|line| {
        let values: Vec<i32> = line.to_string().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        left.push(values[0]);
        right.push(values[1]);
    });

    format!("{}", similarity_score_between_lists(&mut left, &mut right))
}
