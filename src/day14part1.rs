enum Direction {
    N,
    E,
    S,
    W    
}
#[derive(Debug)]
struct Robot{
    v: (i32, i32),
    p: (i32, i32)
}

fn wrap(x:i32, y:i32, width:u32, height:u32) -> (i32, i32){
    ((x % width as i32 + width as i32) % width as i32, (y % height as i32 + height as i32) % height as i32)
}
impl Robot{
    //Premature optimization. Didnt help on the second part.
    fn pos_after(&self, seconds:i32, width:u32, height:u32) -> (i32, i32){
        let x = self.p.0 + self.v.0 * seconds;
        let y = self.p.1 + self.v.1 * seconds;
        wrap(x,y,width, height)
    }
    fn quadrant_after(&self, seconds:i32, width:u32, height:u32) -> Vec<u32>{
        let pos = self.pos_after(seconds, width, height);
        match (match pos.0.cmp(&(width as i32 /2)){
            std::cmp::Ordering::Greater => Some(Direction::E),
            std::cmp::Ordering::Less => Some(Direction::W),
            std::cmp::Ordering::Equal => None
        },
        match pos.1.cmp(&(height as i32 /2)){
            std::cmp::Ordering::Greater => Some(Direction::S),
            std::cmp::Ordering::Less => Some(Direction::N),
            std::cmp::Ordering::Equal => None
        },) {
            (Some(Direction::W), Some(Direction::N)) => vec![1,0,0,0],
            (Some(Direction::W), Some(Direction::S)) => vec![0,0,1,0],
            (Some(Direction::E), Some(Direction::N)) => vec![0,1,0,0],
            (Some(Direction::E), Some(Direction::S)) => vec![0,0,0,1],
            _ => vec![0; 4]
        }
    }
}
fn trim(word:&str) -> &str{
    word.trim_start_matches(['p', 'v', '='])
}

pub fn solution(input: String) -> String { 

    let robots = input
        .lines()
        .map(
            |line| 
            line.split_whitespace()
                .map(
                    |w| 
                    trim(w).split(",")
                                 .map(|x| x.parse::<i32>().unwrap())
                                 .collect::<Vec<i32>>())
                .collect::<Vec<Vec<i32>>>())
        .map(
            |line|
            Robot {p: (line[0][0], line[0][1]), v: (line[1][0], line[1][1])})
        .collect::<Vec<Robot>>();

    let time = 100;
    let (width, height) = if robots.len() < 100 {(11, 7)} else {(101, 103)}; //Test and input differs in the grid size! Change width and height here accordingly.
    format!("{:?}",robots.iter()
                         .map(|robot| robot.quadrant_after(time, width, height))
                         .fold(
                            vec![0; 4], 
                            |acc, v| 
                                acc.iter().zip(v).map(|(x,y)| x+y).collect::<Vec<u32>>())
                         .iter()
                         .product::<u32>())
} 
