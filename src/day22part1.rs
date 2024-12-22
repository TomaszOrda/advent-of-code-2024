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


pub fn solution(input: String) -> String { 
    let initial_secrets = input.lines().map(|line| Seed{ value: line.parse::<u64>().unwrap()}).collect::<Vec<Seed>>();

    format!("{:?}",initial_secrets.iter().map(|secret| secret.iter().nth(2000).unwrap()).sum::<u64>()) 
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
fn test_123_next_10() {
    let results = "15887950
                           16495136
                           527345
                           704524
                           1553684
                           12683156
                           11100544
                           12249484
                           7753432
                           5908254".chars().filter(|&c| c!=' ').collect::<String>();
    let secret = Seed {value: 123};
    secret.iter().skip(1).take(results.len()).zip(results.split_whitespace().collect::<Vec<&str>>()).for_each(|(number, result)| assert_eq!(number, result.parse::<u64>().unwrap()))
}#[test]
fn basic_test() {
    let input = "1
                         10
                         100
                         2024".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input).parse::<u64>().unwrap(), 37327623)
}

