use std::collections::{BTreeSet, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    fn add(&self, other: (char, char), edges: &HashMap<(char, char), BTreeSet<(char, char)>>) -> Option<Clique>{
        if self.computers.is_subset(edges.get(&other).unwrap()){
            let mut new_computers = self.computers.clone();
            new_computers.insert(other);
            Some(Self { computers: new_computers })
        }else{
            None
        }
    }
    fn to_password(&self)-> String{
        self.computers.iter().map(|(a,b)| format!("{a}{b}")).collect::<Vec<String>>().join(",")
    }
}
//Works in 6-7 seconds on my machine. It is an NP-complete problem. There are algorithms on wiki that solve it faster. And there are likely easy ways to make this fast enough.
pub fn solution(input: String) -> String { 
    let connections = input
        .lines()
        .map(Clique::new)
        .collect::<BTreeSet<Clique>>();

    let mut edges: HashMap<(char, char), BTreeSet<(char, char)>> = HashMap::new();
    connections
        .iter()
        .for_each(
            |c|
            {
                edges.entry(*c.computers.first().unwrap())
                     .or_default()
                     .insert(*c.computers.last().unwrap());
            });

    let computers = edges.keys().copied().collect::<Vec<(char, char)>>();
    
    let mut result = connections.clone();
    while result.len()>1 {
        result = result
            .iter()
            .flat_map(
                |clique|
                computers
                    .iter()
                    .filter_map(|other| clique.add(*other, &edges))
                    .collect::<Vec<Clique>>())
            .collect()
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

