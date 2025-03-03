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
    fn push_box_side(&mut self, pos: (usize,usize), direction:&Direction) -> Option<()>{
        let step1 = ((pos.0 as i8 + direction.to_vec().0) as usize, (pos.1 as i8 + direction.to_vec().1) as usize);
        let step2 = ((pos.0 as i8 + 2*direction.to_vec().0) as usize, (pos.1 as i8 + 2*direction.to_vec().1) as usize);
        match self.grid[step2.0][step2.1] {
            '.' => {
                self.grid[step2.0][step2.1] = self.grid[step1.0][step1.1];
                self.grid[step1.0][step1.1] = self.grid[pos.0][pos.1];
                self.grid[pos.0][pos.1] = '.';
                Some(())
            },
            '[' | ']' => {
                match self.push_box_side(step2, direction){
                    Some(()) => {
                        self.grid[step2.0][step2.1] = self.grid[step1.0][step1.1];
                        self.grid[step1.0][step1.1] = self.grid[pos.0][pos.1];
                        self.grid[pos.0][pos.1] = '.';
                        Some(())
                    }
                    None => None
                }
            },
            '#' => {
                None
            },
            _   => panic!("Unexpected token on the map!")
        }
    }
    fn can_push_box(&self, x:usize ,y:usize, direction:&Direction) -> bool{
        let stepy = (y as i8 + direction.to_vec().0) as usize;
        let (left, right) = (self.grid[stepy][x],self.grid[stepy][x+1]);

        (match left {
            ']' => self.can_push_box(x-1_usize,  stepy, direction ),
            '[' => self.can_push_box(x,  stepy, direction ),
            '.' => true,
            '#' => false,
            _ => panic!("Unexpected map token!")
        }) 
        &&
        (match right {
            '[' => self.can_push_box(x +1,  stepy, direction ),
            ']' => true,
            '.' => true,
            '#' => false,
            _ => panic!("Unexpected map token!")
        })
    }
    fn push_box_top_down(&mut self, x: usize, y:usize, direction:&Direction){
        let stepy = (y as i8 + direction.to_vec().0) as usize;
        let (left, right) = (self.grid[stepy][x],self.grid[stepy][x+1]);
        // dbg!(x,x+1,y,left, right);
        match left {
            ']' => self.push_box_top_down(x-1_usize, stepy, direction ),
            '[' => self.push_box_top_down(x,stepy, direction ),
            '.' => (),
            '#' => panic!("Wall encountered"),
            _ => panic!("Unexpected map token!")
        };
        match right {
            '[' => self.push_box_top_down(x+1, stepy, direction ),
            ']' => (),
            '.' => (),
            '#' => panic!("Wall encountered"),
            _ => panic!("Unexpected map token!")
        };
        self.grid[stepy][x] = self.grid[y][x];
        self.grid[stepy][x+1] = self.grid[y][x+1];
        self.grid[y][x+1] = '.';
        self.grid[y][x] = '.';
    }
    fn apply_move(&mut self, direction:&Direction){
        let new_pos = ((self.robot.0 as i8 + direction.to_vec().0) as usize, (self.robot.1 as i8 + direction.to_vec().1) as usize);
        self.grid[self.robot.0][self.robot.1] = '.';
        match self.grid[new_pos.0][new_pos.1] {
            '.' => self.robot = new_pos,
            '[' => match direction{
                Direction::E | Direction::W  => {
                    if let Some(()) = self.push_box_side(new_pos, direction) { self.robot = new_pos }
                },
                
                Direction::N | Direction::S  => {
                    if self.can_push_box(new_pos.1, new_pos.0, direction){
                        self.push_box_top_down(new_pos.1, new_pos.0, direction);
                        self.robot = new_pos;
                    }
                } 
            }
            ']' => match direction{
                Direction::E | Direction::W  => {
                    if let Some(()) = self.push_box_side(new_pos, direction) { self.robot = new_pos }
                },
                
                Direction::N | Direction::S  => {
                    if self.can_push_box(new_pos.1-1, new_pos.0, direction){
                        self.push_box_top_down(new_pos.1-1, new_pos.0, direction);
                        self.robot = new_pos;
                    }
                } 
            }
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
                 .flat_map(
                    |line| 
                    line.1.iter()
                          .enumerate()
                          .filter(|c| c.1==&'[')
                          .map(move |c| Self::gps_coordinate(line.0, c.0)))
                 .sum::<u32>()
    }
}



pub fn solution(input: String) -> String { 
    let lines = input.lines().map(|line| line.chars().collect::<Vec<char>>());
    let (grid, moves): (Vec<Vec<char>>, Vec<Vec<char>>) = lines.partition(|line| line.starts_with(&['#']));
    let grid = grid
        .iter()
        .map(
            |line| 
            line.iter()
                .flat_map(
                    |c| 
                    match c{
                        '.' => vec!['.','.'],
                        '#' => vec!['#','#'],
                        'O' => vec!['[',']'],
                        '@' => vec!['@','.'],
                        _ => panic!("Invalid map token")
                    })
                .collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let moves = moves.iter().flatten().map(|&arrow| Direction::new(arrow)).collect::<Vec<Direction>>();
    let robot_flat = grid.iter().flatten().position(|c| c == &'@').unwrap();
    let mut map = Map {
        robot: (robot_flat / grid[0].len() , robot_flat % grid[0].len()),
        grid,
    };
    moves.iter().for_each(|direction| map.apply_move(direction));
    format!("{:?}",map.sum_of_gps_coordinates())
    // format!("{:?}",map.grid.iter().map(|line| line.iter().collect::<String>()).collect::<Vec<String>>().join("\n")) 
} 
