
use regex::Regex;

fn extract_multiplication_pairs_and_apply_dos(memory: String) -> Vec<(u32, u32)>{
    //I could do it simmilar way as the previous task (another partitioning by do and dont and then flatmap), but I decided to train some regex instead.
    let re = Regex::new(r"(?i)mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap(); 
    let mut enabled = true;
    let commands =  re.find_iter(&memory).map(|r| r.as_str().to_string()).map(|command|{
        match command.as_str() {
            "do()" => {enabled = true; "!".to_string()},
            "don't()" => {enabled = false; "!".to_string()},
            mul => if enabled {mul.to_string()} else {"!".to_string()}
        }
    }).filter(|command| command!="!").collect::<Vec<String>>();
    commands.iter()
            .map(|command| command.strip_prefix("mul(")
                                            .unwrap()
                                            .strip_suffix(")")
                                            .unwrap())
            .map(|sub|  {
                        let mut left_right = sub.split(",");
                        (left_right.next().unwrap().to_string(), left_right.next().unwrap_or("!").to_string())
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
    
    format!("{:?}",extract_multiplication_pairs_and_apply_dos(input).iter().map(|(a,b)| a*b ).sum::<u32>())
}
