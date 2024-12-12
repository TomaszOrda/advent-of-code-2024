fn stones_after_n_blinks(stone:u64, blinks_left:u64) -> u64{
    if blinks_left == 0{
        return 1
    }
    match stone {
        0 => stones_after_n_blinks(1, blinks_left-1),
        d => {
            let len = d.to_string().len();
            if len%2 == 1 {
                stones_after_n_blinks(stone*2024, blinks_left-1)
            }else{
                stones_after_n_blinks(stone.to_string()[..len/2].parse::<u64>().unwrap(), blinks_left-1)
                +
                stones_after_n_blinks(stone.to_string()[len/2..].parse::<u64>().unwrap(), blinks_left-1)
            
            }
        }
    }
}

pub fn solution(input: String) -> String { 
    let stones = input.split_whitespace()
                                .map(|w| w.parse::<u64>().unwrap())
                                .collect::<Vec<u64>>();

    format!("{:?}",stones.iter().map(|&stone| stones_after_n_blinks(stone, 25)).sum::<u64>()) 
} 
