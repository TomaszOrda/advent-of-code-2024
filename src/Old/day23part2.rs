use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
struct Clique{
    computers:BTreeSet<(char, char)>
}
impl Clique{
    fn new(line: &str) -> Self{
        let line = line.chars().collect::<Vec<char>>();
        let left  = (line[0], line[1]);
        let right = (line[3], line[4]);
        if line[0..2]<line[3..5]{
            Self { computers:[left, right].into() }
        }else{
            Self { computers:[right, left].into() }
        }
    }
    fn add(&self, other: &Clique, connections: &BTreeSet<Clique>) -> Option<Clique>{
        if  connections.contains(&Clique{computers: [*self.computers.first().unwrap(), *other.computers.last().unwrap()].into()}) &&
            self.computers.intersection(&other.computers).count() == self.computers.len()-1 && 
            self.computers.symmetric_difference(&other.computers).collect::<Vec<&(char, char)>>() == [self.computers.first().unwrap(), other.computers.last().unwrap()]
        {
            let mut new_computers = self.computers.clone();
            new_computers.insert(*other.computers.last().unwrap());
            Some(Self { computers: new_computers })
        }else{
            None
        }
    }
    fn to_password(&self)-> String{
        self.computers.iter().map(|(a,b)| format!("{a}{b}")).collect::<Vec<String>>().join(",")
    }
}
//Certainly not fast enough
pub fn solution(input: String) -> String { 
    let connections = input
        .lines()
        .map(Clique::new)
        .collect::<BTreeSet<Clique>>();
    let mut result = connections.clone();
    loop {
        dbg!(&result.first().unwrap().computers.len());
        let new_result = result
            .iter()
            .flat_map(
                |clique|
                result
                    .iter()
                    .filter_map(
                        |other| 
                        clique.add(other, &connections))
                    .collect::<Vec<Clique>>())
            .collect::<BTreeSet<Clique>>();
        if new_result.is_empty(){
            break
        }else{
            result = new_result
        }
    }
    result.first().unwrap().to_password()
} 
#[test]
fn basic_test() {
    let input = "ka-co
                        ta-co
                        de-co
                        ta-ka
                        de-ta
                        ka-de".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input), "co,de,ka,ta".to_string())
}

