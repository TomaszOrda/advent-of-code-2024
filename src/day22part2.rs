use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Seed{
    value: u64
}
impl Seed{
    pub fn iter(&self) ->  impl Iterator<Item = u64> {
        std::iter::successors(Some(self.value), |&n| Self::next_secret(n))
    }
    fn next_secret(number: u64) -> Option<u64>{
        let phase_1 = Self::mix_prune(number , number  * 64  );
        let phase_2 = Self::mix_prune(phase_1, phase_1 / 32  );
        let phase_3 = Self::mix_prune(phase_2, phase_2 * 2048);
        Some(phase_3)
    } 
    fn mix_prune(secret_number:u64, mix_in: u64) -> u64{
        Self::prune(Self::mix(secret_number, mix_in))
    }
    fn mix(secret_number:u64, mix_in: u64) ->u64{
        secret_number ^ mix_in
    }
    fn prune(number:u64) -> u64{
        number % 16_777_216
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct PriceChangeWindow{
    w: (i8,i8,i8,i8)
}
impl PriceChangeWindow{
    fn new(v: &[i8]) -> Self{
        PriceChangeWindow {w:(v[0], v[1], v[2], v[3])}
    }
}

fn merge_hashmaps_sum(hm1: HashMap<PriceChangeWindow, u32>, hm2: HashMap<PriceChangeWindow, u32>) -> HashMap<PriceChangeWindow, u32>{
    let mut result = hm1;
    for (key, value) in hm2{
        *result.entry(key).or_insert(0) += value;
    }
    result
}

pub fn solution(input: String) -> String { 
    let initial_secrets = input.lines().map(|line| Seed{ value: line.parse::<u64>().unwrap()}).collect::<Vec<Seed>>();

    let buyers_prices = initial_secrets
        .iter()
        .map(
            |secret| 
            secret.iter().take(2000))
        .map(
            |numbers|
            numbers.map(|number| (number % 10) as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();
        
    let buyers_price_changes = buyers_prices
        .iter()
        .map(
            |prices|
            prices.windows(2).map(|seq| seq[1] as i8 - seq[0] as i8).collect::<Vec<i8>>())
        .collect::<Vec<Vec<i8>>>();
    
    let buyers_sequence_values = buyers_price_changes
        .iter()
        .map(
            |price_changes|
            price_changes
                .windows(4)
                .map(PriceChangeWindow::new))
        .zip(buyers_prices)
        .map(
            |(windows, prices)|
            windows.zip(prices.into_iter().skip(4).map(|p| p as u32)).rev().collect::<HashMap<PriceChangeWindow, u32>>()
        )
        .collect::<Vec<HashMap<PriceChangeWindow, u32>>>();

    let global_sequence_values = buyers_sequence_values
        .into_iter()
        .fold(HashMap::new(), merge_hashmaps_sum);

    format!("{:?}",global_sequence_values.values().max().unwrap()) 
} 
#[test]
fn test_mix() {
    assert_eq!(Seed::mix(42, 15), 37)
}
#[test]
fn test_prune() {
    assert_eq!(Seed::prune(100000000), 16113920)
}
#[test]
fn test_mix_prune() {
    let number = 353684523;
    assert_eq!(Seed::mix_prune(number, 4572), Seed::prune(Seed::mix(number, 4572)))
}
#[test]
fn basic_test() {
    let input = "1
                         2
                         3
                         2024".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input).parse::<u64>().unwrap(), 23)
}

