enum Direction{
    N,
    E,
    S,
    W
}
impl Direction{
    fn to_vec(&self) -> (i8, i8){
        match self{
            Self::N => (-1,  0),
            Self::E => ( 0,  1),
            Self::S => ( 1,  0),
            Self::W => ( 0, -1),
        }
    }
    fn new(arrow:char) -> Self{
        match arrow{
            '<' => Self::W, 
            '>' => Self::E,
            'v' => Self::S,
            '^' => Self::N,
            _   => panic!("Invalid arrow!")
        }
    }
}

struct Map{
    grid: Vec<Vec<char>>,
    robot: (usize, usize)
}
impl Map {
    fn push_box(&mut self, pos: (usize,usize), direction:&Direction) -> Option<()>{
        let new_pos = ((pos.0 as i8 + direction.to_vec().0) as usize, (pos.1 as i8 + direction.to_vec().1) as usize);
        match self.grid[new_pos.0][new_pos.1] {
            '.' => {
                self.grid[new_pos.0][new_pos.1] = 'O';
                return Some(())
            },
            'O' => self.push_box(new_pos, direction),
            '#' => {
                return None
            },
            _   => panic!("Unexpected token on the map!")
        }
    }
    fn apply_move(&mut self, direction:&Direction){
        let new_pos = ((self.robot.0 as i8 + direction.to_vec().0) as usize, (self.robot.1 as i8 + direction.to_vec().1) as usize);
        self.grid[self.robot.0][self.robot.1] = '.';
        match self.grid[new_pos.0][new_pos.1] {
            '.' => self.robot = new_pos,
            'O' => match self.push_box(new_pos, direction){
                Some(()) => self.robot = new_pos,
                None => ()
            },
            '#' => (),
            _   => panic!("Unexpected token on the map!")
        }
        self.grid[self.robot.0][self.robot.1] = '@';
    }
    fn gps_coordinate(y:usize, x:usize) -> u32{
        (y * 100 + x)  as u32
    }
    fn sum_of_gps_coordinates(&self) -> u32{
        self.grid.iter()
                 .enumerate()
                 .map(
                    |line| 
                    line.1.iter()
                          .enumerate()
                          .filter(|c| c.1==&'O')
                          .map(move |c| Self::gps_coordinate(line.0, c.0)))
                 .flatten()
                 .sum::<u32>()
    }
}



pub fn solution(input: String) -> String { 
    let lines = input.lines().map(|line| line.chars().collect::<Vec<char>>());
    let (grid, moves): (Vec<Vec<char>>, Vec<Vec<char>>) = lines.partition(|line| line.starts_with(&['#']));
    let moves = moves.iter().flatten().map(|&arrow| Direction::new(arrow)).collect::<Vec<Direction>>();
    let robot_flat = grid.iter().flatten().position(|c| c == &'@').unwrap();
    let mut map = Map {
        robot: (robot_flat / grid[0].len() , robot_flat % grid[0].len()),
        grid,
    };
    moves.iter().for_each(|direction| map.apply_move(&direction));
    format!("{:?}",map.sum_of_gps_coordinates()) 
} 
