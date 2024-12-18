struct Grid<T>{
    map: Vec<Vec<T>>,
}
impl<T> Grid<T> {
    fn new(map: Vec<Vec<T>>) -> Self{
        Self{
            map,
        }
    }
    fn get(&self, x:i32, y:i32) -> Option<&T>{
        if x<0 || y<0{
            return None
        }
        if let Some(v) = self.map.get(y as usize){
            v.get(x as usize)
        }else{
            None
        }
    }
    fn get_mut(&mut self, x:i32, y:i32) -> Option<&mut T>{
        if x<0 || y<0{
            return None
        }
        if let Some(v) = self.map.get_mut(y as usize){
            v.get_mut(x as usize)
        }else{
            None
        }
    }
}


fn bfs(grid: &Grid<char>, start: (i32, i32), end: (i32, i32)) -> u32{
    let mut distances = Grid::new(vec![vec![u32::MAX; grid.map[0].len()]; grid.map.len()]);
    *distances.get_mut(start.0, start.1).unwrap() = 0;
    let mut stack = vec![start];
    while let Some(position) = stack.pop(){
        let current_distance = *distances.get(position.0,position.1).unwrap();
        for dir in [(1,0), (-1,0), (0,1), (0,-1)]{
            let (x,y) = (position.0 + dir.0, position.1 + dir.1);
            if let Some(&map_element) = grid.get(x,y){
                if map_element == '.' && *distances.get(x,y).unwrap()>current_distance+1{
                    *distances.get_mut(x, y).unwrap() = current_distance+1;
                    stack.insert(0, (x,y))
                }
            }
        }
    }
    *distances.get(end.0, end.1).unwrap()
}

pub fn solution(input: String) -> String { 
    let bytes_coords = input
        .lines()
        .map(
            |line| 
            line.split(",")
                .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>() )
        .collect::<Vec<Vec<usize>>>();
    
    let (width, height) = if bytes_coords.len() < 100 {(7, 7)} else {(71, 71)}; //Test and input differs in the grid size! Change width and height here accordingly.
    let number_of_bytes_to_fall  = if bytes_coords.len() < 100 {12}     else {1024}; 
    
    let mut map = vec![vec!['.'; width]; height];
    bytes_coords[0..number_of_bytes_to_fall].iter().for_each(|coords| map[coords[1]][coords[0]] = '#');

    format!("{:?}",bfs(&Grid::new(map), (0,0), (width as i32 -1, height as i32 -1))) 
} 
