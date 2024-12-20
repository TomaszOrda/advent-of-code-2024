#[derive(Debug, Clone)]
struct Grid<T> {
    map: Vec<Vec<T>>,
    height: i32,
    width: i32
}
impl<T> Grid<T> {
    fn new(map: Vec<Vec<T>>) -> Self {
        Self { height:map.len() as i32, width:map[0].len() as i32, map }
    }
    fn get(&self, x: i32, y: i32) -> Option<&T> {
        if x < 0 || y < 0 {
            return None;
        }
        if let Some(v) = self.map.get(y as usize) {
            v.get(x as usize)
        } else {
            None
        }
    }
    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        if x < 0 || y < 0 {
            return None;
        }
        if let Some(v) = self.map.get_mut(y as usize) {
            v.get_mut(x as usize)
        } else {
            None
        }
    }
}

impl<T: std::cmp::PartialEq> Grid<T> {
    fn position(&self, predicate: fn(&T)->bool ) -> (i32, i32) {
        let position_flat = self.map.iter().flatten().position(predicate).unwrap() as i32;
        (position_flat % self.width, position_flat / self.width)
    }
}


fn track_times(grid: &Grid<char>) -> Grid<Option<u32>>{
    let mut grid_track_times = Grid::new(vec![vec![None; grid.width as usize]; grid.height as usize]);
    let start = grid.position(|&c| c == 'S');
    let end = grid.position(|&c| c == 'E');
    let mut position = start;
    let mut time = 0;
    *grid_track_times.get_mut(start.0, start.1).unwrap() = Some(time);
    while position != end{
        let next_position = [(-1,0), (1,0), (0,-1), (0,1)]
            .iter()
            .map( |v| (position.0 + v.0, position.1 + v.1))
            .find(
                |new_pos| 
                grid.get(new_pos.0, new_pos.1).unwrap() != &'#' && 
                grid_track_times.get(new_pos.0, new_pos.1).unwrap().is_none())
            .unwrap();
        time +=1;
        *grid_track_times.get_mut(next_position.0, next_position.1).unwrap() = Some(time);
        position = next_position;
    }
    grid_track_times
}

fn total_cheats(grid_track_times: &Grid<Option<u32>>, position: (i32, i32), min_skip: u32) -> u32{
    let current_time = match grid_track_times.get(position.0, position.1).unwrap(){
        Some(x) => x,
        None => return 0
    };
    [(-2,0), (2,0), (0,-2), (0,2), (-1,-1), (1,1), (1,-1), (-1,1)]
        .iter()
        .map( |v| (position.0 + v.0, position.1 + v.1))
        .filter(
            |new_pos| 
            //Nested options are not the pretties thing. I guess I should've written a new structs for this day.
            match grid_track_times.get(new_pos.0, new_pos.1){
                Some(Some(track_time_after_cheat)) => *track_time_after_cheat>= current_time+2+min_skip,
                _ => false
            })
        .count() as u32
}

pub fn solution(input: String) -> String {
    let grid = Grid::new(
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>(),
    );
    let min_skip = if grid.height < 50 {8} else {100}; //For the test case we assume any path that saves >=8ps, the solution should return 14
    let grid_track_times = track_times(&grid);

    format!("{:?}", (0..grid.width).flat_map(|x| (0..grid.height).map(move |y| (x,y))).map(|pos| total_cheats(&grid_track_times, pos, min_skip)).sum::<u32>())
}

#[test]
fn basic_test() {
    let input = "###############
                         #...#...#.....# 
                         #.#.#.#.#.###.# 
                         #S#...#.#.#...# 
                         #######.#.#.### 
                         #######.#.#...# 
                         #######.#.###.# 
                         ###..E#...#...# 
                         ###.#######.### 
                         #...###...#...# 
                         #.#####.#.###.# 
                         #.#...#.#.#...# 
                         #.#.#.#.#.#.### 
                         #...#...#...### 
                         ###############".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input).parse::<u32>().unwrap(), 14)
}
