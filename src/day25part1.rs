#[derive(Debug, PartialEq)]
enum SchematicType{
    Key,
    Lock
}
struct Schematic{
    kind: SchematicType,
    _drawing: Vec<Vec<char>>,
    pins: Vec<u8>
}
impl Schematic {
    fn new(drawing: &[Vec<char>]) -> Self{
        let kind = if drawing[0][0] == '#' {SchematicType::Lock} else {SchematicType::Key};
        let pins = drawing
            .iter()
            .skip(1)
            .rev()
            .skip(1)
            .rev()
            .fold(
                vec![0;5], 
                |acc, row| 
                {
                    row.iter().map(|c| (c==&'#') as u8).zip(acc).map(|(x,y)| x+y).collect::<Vec<u8>>()//
                });
        Self { kind, _drawing:drawing.to_vec(), pins }
    }
    fn fits(&self, other:&Schematic) -> bool{
        self.kind != other.kind &&
        self.pins.iter().zip(other.pins.iter()).all(|(pin1, pin2)| pin1 + pin2 <= 5)
    }
}
pub fn solution(input: String) -> String { 
    let schematics = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
        .chunks(7)
        .map(Schematic::new)
        .collect::<Vec<Schematic>>();

    let (keys, locks): (Vec<Schematic>, Vec<Schematic>) = schematics
        .into_iter()
        .partition(
            |schematic|
            matches!(schematic.kind, SchematicType::Key));
            
    format!("{:?}",keys.iter().flat_map(|key| locks.iter().zip(std::iter::repeat(key) )).filter(|(key, lock)| key.fits(lock)).count()) 
} 
#[test]
fn basic_test_overlap() {
    let input = "#####
                         ##.##
                         .#.##
                         ...##
                         ...#.
                         ...#.
                         .....
                         
                         .....
                         #....
                         #....
                         #...#
                         #.#.#
                         #.###
                         #####".lines().map(|line| line.trim().to_string()).collect::<Vec<String>>().join("\r\n");
    assert_eq!(solution(input).parse::<u64>().unwrap(), 0)
}


#[test]
fn basic_test_fit() {
    let input = "#####
                         ##.##
                         .#.##
                         ...##
                         ...#.
                         ...#.
                         .....
                         
                         .....
                         ..#..
                         #.#..
                         ###..
                         ###.#
                         ###.#
                         #####".lines().map(|line| line.trim().to_string()).collect::<Vec<String>>().join("\r\n");
    assert_eq!(solution(input).parse::<u64>().unwrap(), 1)
}


#[test]
fn basic_test_shaved() {
    let input = "#####
                         ##.##
                         .#...
                         .....
                         .....
                         .....
                         .....
                         
                         .....
                         .....
                         .....
                         .....
                         #...#
                         #.###
                         #####".lines().map(|line| line.trim().to_string()).collect::<Vec<String>>().join("\r\n");
    assert_eq!(solution(input).parse::<u64>().unwrap(), 1)
}

