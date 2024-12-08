

fn extract_multiplication_pairs(memory: String) -> Vec<(u32,u32)>{

    memory.split("mul(")
        .map(|sub|  
                sub.split(")")
                    .next()
                    .unwrap())
        .map(|sub|  {
                let mut left_right = sub.split(",");
                let res = (left_right.next().unwrap(), left_right.next().unwrap_or("!"));
                if left_right.next().is_some() {("!", "!")} else {res} //a special check for when there are more than two comma separated sections found
        })
        .filter_map(|pair| {
            let left  = pair.0.parse::<u32>();
            let right = pair.1.parse::<u32>();
            match (left, right){
                (l,r) if l.is_ok() && r.is_ok() => Some((l.unwrap(),r.unwrap())),
                _ => None
            }
        })
        .collect::<Vec<(u32, u32)>>()
}

pub fn solution(input: String) -> String {
    format!("{:?}",extract_multiplication_pairs(input).iter().map(|(a,b)| a*b ).sum::<u32>())
}
