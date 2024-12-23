#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Connection{
    left: (char, char),
    right: (char, char)
}
#[derive(Debug)]
struct PairedConnection{
    left: (char, char),
    mid: (char, char),
    right: (char, char)
}
impl Connection{
    fn new(line: &str) -> Self{
        let line = line.chars().collect::<Vec<char>>();
        let left  = (line[0], line[1]);
        let right = (line[3], line[4]);
        if line[0..2]<line[3..5]{
            Self { left      , right       }
        }else{
            Self { left:right, right:left  }
        }
    }
}
impl PairedConnection{
    fn new(left_connection: &Connection, right_connection: &Connection) -> Self{
        Self { left:left_connection.left , mid: left_connection.right, right: right_connection.right }
    }
    fn might_include_chef_historian(&self) -> bool{
        self.left.0 == 't' || self.mid.0 == 't' || self.right.0 == 't' 
    }
}

pub fn solution(input: String) -> String { 
    let connections = input
        .lines()
        .map(Connection::new)
        .collect::<Vec<Connection>>();
    let paired_connections = connections
        .iter()
        .flat_map(
            |connection|
            connections
                .iter()
                .filter_map(
                    |other| 
                    if connection.right == other.left 
                    {Some(PairedConnection::new(connection, other))} 
                    else {None})
                .collect::<Vec<PairedConnection>>())
        .collect::<Vec<PairedConnection>>();
    let triangles = paired_connections
        .into_iter()
        .filter(
            |connection3|
            connections.iter().any(|c| c == &Connection{left: connection3.left, right:connection3.right}))
        .collect::<Vec<PairedConnection>>();
    format!("{:?}",triangles.iter().filter(|t| t.might_include_chef_historian()).count()) 
    // format!("{:?}",triangles.into_iter().filter(|t| t.might_include_chef_historian()).collect::<Vec<PairedConnection>>()) 
} 
#[test]
fn test_empty() {
    let input = "kh-tc
                        qp-kh
                        de-cg".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input).parse::<u32>().unwrap(), 0)
}

#[test]
fn test1() {
    let input = "wh-tc
                        tc-td
                        wh-td".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input).parse::<u32>().unwrap(), 1)
}
#[test]
fn test2() {
    let input = "wh-tc
                        tb-ka
                        tc-td
                        wh-td".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input).parse::<u32>().unwrap(), 1)
}

