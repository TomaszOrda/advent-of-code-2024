use std::collections::HashMap;

//this length is faster as expected (experience from day 7)
//Not sure if it contributed much into the final solution
fn len(stone:u64) -> u32{
    let mut len = 0;
    let mut ten_pow_len = 1;
    while ten_pow_len<=stone{
        ten_pow_len*=10;
        len+=1;
    }
    len
}


fn split(stone:u64)->Option<(u64,u64)>{
    let len = len(stone);
    if len%2 == 1 {
        None
    }else{
        let divisor = 10_u64.pow(len/2);
        Some( (stone / divisor, 
               stone % divisor) )
    }

}

//35 blinks, cache made time drop from 166ms to 10 ms
//And thus solved the 75 blinks in 75ms (sic!) which is very acceptable
struct Cache{
    memory: HashMap<(u64, u64), u64>
}
impl Cache{
    fn new() -> Self{
        Self { memory: HashMap::new() }
    }
    fn get(&mut self, stone:u64, blinks_left:u64) -> u64{
        if blinks_left == 0{
            return 1
        }
        if let Some(cached_value) = self.memory.get(&(stone, blinks_left)){
            *cached_value
        }else{
            let calculation = match stone {
                0 => self.get(1, blinks_left-1),
                _ => {
                    if let Some(res) = split(stone){
                        self.get(res.0, blinks_left-1)
                        +
                        self.get(res.1, blinks_left-1)
                    }else{
                        self.get(stone*2024, blinks_left-1)
                    }
                }
            };
            self.memory.insert((stone, blinks_left), calculation);
            calculation
        }
    }
}


pub fn solution(input: String) -> String { 
    let stones = input.split_whitespace()
                                .map(|w| w.parse::<u64>().unwrap())
                                .collect::<Vec<u64>>();

    let mut cache = Cache::new();
    format!("{:?}",stones.iter().map(|&stone| cache.get(stone, 75)).sum::<u64>()) 
} 
