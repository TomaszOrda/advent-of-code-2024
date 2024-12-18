//I know I could optimize this solution, but it works under a second (using --release). Changing from distances to reachable is good enough optimization.
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


fn can_reach_end(grid: &Grid<char>, start: (i32, i32), end: (i32, i32)) -> bool{
    //We are not concerned about the distances now. Part 1 is somewhat of a red herring here (though distances would work).
    let mut reachable = Grid::new(vec![vec![false; grid.map[0].len()]; grid.map.len()]);
    *reachable.get_mut(start.0, start.1).unwrap() = true;
    let mut stack = vec![start];
    while let Some(position) = stack.pop(){
        for dir in [(1,0), (-1,0), (0,1), (0,-1)]{
            let (x,y) = (position.0 + dir.0, position.1 + dir.1);
            if let Some(&map_element) = grid.get(x,y){
                if map_element == '.' && !reachable.get(x,y).unwrap(){
                    *reachable.get_mut(x, y).unwrap() = true;
                    stack.insert(0, (x,y))
                }
            }
        }
    }
    *reachable.get(end.0, end.1).unwrap()
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

    let map = vec![vec!['.'; width]; height];
    let mut grid = Grid::new(map);
    let mut next_byte = 0;
    while can_reach_end(&grid, (0,0), (width as i32 -1, height as i32 -1)){
        let byte_coord = &bytes_coords[next_byte];
        next_byte +=1;
        *grid.get_mut(byte_coord[0] as i32, byte_coord[1] as i32).unwrap() = '#';
    }
    format!("{:?}",bytes_coords[next_byte-1].iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")) 
} 
