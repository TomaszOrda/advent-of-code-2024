//My honest try at making previous code better
//Required more debugging i think. However it does is more readable.
//I could implement move_towards or something simmilar for direction.

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
    fn can_push_box(&self, y:usize, x:usize, direction:&Direction) -> bool{
        match direction {
            Direction::E => {
                let step_ee = x + 2;
                match self.grid[y][step_ee] {
                    '.' => true,
                    '[' => self.can_push_box(y, step_ee, direction),
                    '#' => false,
                    ']' => panic!("Box fractioned"),
                    _   => panic!("Unexpected token on the map!")
                }
            },
            
            Direction::W  => {
                let step_w = x - 1;
                let step_ww = x - 2;
                match self.grid[y][step_w] {
                    '.' => true,
                    ']' => self.can_push_box(y, step_ww, direction),
                    '[' => panic!("Box fractioned"),
                    '#' => false,
                    _   => panic!("Unexpected token on the map!")
                }
            },
            Direction::N | Direction::S => {
                let step_y = (y as i8 + direction.to_vec().0) as usize;
                let (left, right) = (self.grid[step_y][x],self.grid[step_y][x+1]);
                (match left {
                    ']' => self.can_push_box(step_y, x-1, direction),
                    '[' => self.can_push_box(step_y, x, direction ),
                    '.' => true,
                    '#' => false,
                    _ => panic!("Unexpected map token!")
                }) 
                &&
                (match right {
                    '[' => self.can_push_box(step_y, x + 1, direction),
                    ']' => true,
                    '.' => true,
                    '#' => false,
                    _ => panic!("Unexpected map token!")
                })
            }
        }
        

        
    }
    fn push_box(&mut self, y:usize, x: usize, direction:&Direction){
        match direction {
            Direction::E  => {
                let step_e  = x + 1;
                let step_ee = x + 2;
                match self.grid[y][step_ee] {
                    '.' => (),
                    '[' => self.push_box(y, step_ee, direction),
                    '#' => panic!("Wall encountered"),
                    _   => panic!("Unexpected token on the map!")
                };
                
                self.grid[y][step_e] = '[';
                self.grid[y][step_ee] = ']';
                self.grid[y][x] = '.';
            },
            Direction::W  => {
                let step_w  = x - 1;
                let step_ww = x - 2;
                match self.grid[y][step_w] {
                    '.' => (),
                    ']' => self.push_box(y, step_ww, direction),
                    '#' => panic!("Wall encountered"),
                    '[' => panic!("Box fractioned"), 
                    _   => panic!("Unexpected token on the map!")
                };
                
                self.grid[y][x] = ']';
                self.grid[y][step_w] = '[';
                self.grid[y][x+1] = '.';
            },
            Direction::N | Direction::S => {
                let step_y = (y as i8 + direction.to_vec().0) as usize;
                let (left, right) = (self.grid[step_y][x],self.grid[step_y][x+1]);
                match left {
                    ']' => self.push_box(step_y, x-1, direction ),
                    '[' => self.push_box(step_y, x, direction),
                    '.' => (),
                    '#' => panic!("Wall encountered"),
                    _ => panic!("Unexpected map token!")
                };
                match right {
                    '[' => self.push_box(step_y, x+1, direction ),
                    ']' => (),
                    '.' => (),
                    '#' => panic!("Wall encountered"),
                    _ => panic!("Unexpected map token!")
                };
                self.grid[step_y][x] = self.grid[y][x];
                self.grid[step_y][x+1] = self.grid[y][x+1];
                self.grid[y][x+1] = '.';
                self.grid[y][x] = '.';
            }
        }
    }
    fn apply_move(&mut self, direction:&Direction){
        let new_pos = ((self.robot.0 as i8 + direction.to_vec().0) as usize, (self.robot.1 as i8 + direction.to_vec().1) as usize);
        self.grid[self.robot.0][self.robot.1] = '.';
        match self.grid[new_pos.0][new_pos.1] {
            '.' => self.robot = new_pos,
            '[' => if self.can_push_box(new_pos.0, new_pos.1, direction){
                self.push_box(new_pos.0, new_pos.1, direction);
                self.robot = new_pos;
            }else{
            }
            ']' => if self.can_push_box(new_pos.0, new_pos.1-1, direction){
                    self.push_box(new_pos.0, new_pos.1-1, direction);
                    self.robot = new_pos;
                }else{
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
