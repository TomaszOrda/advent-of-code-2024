//This one was tedious
//However i did get around implementing my own grid (which was easier than i thought), maybe ill export it as an util (that is if i need it for a future task)
//It was an exercise in writing big libraries I feel like. Good experience to have; Ungrateful labor at the same time.
//I could've used grid/mattrix and bfs from external crate. Maybe i will next time.
struct Grid<T>{
    map: Vec<Vec<T>>,
}
impl<T> Grid<T> {
    fn new(map: Vec<Vec<T>>) -> Self{
        Self{
            map,
        }
    }
    fn get(&self, x:usize, y:usize) -> Option<&T>{
        if let Some(v) = self.map.get(y){
            v.get(x)
        }else{
            None
        }
    }
    fn get_mut(&mut self, x:usize, y:usize) -> Option<&mut T>{
        if let Some(v) = self.map.get_mut(y){
            v.get_mut(x)
        }else{
            None
        }
    }
}
#[derive(PartialEq)]
struct Plant{
    x:usize,
    y:usize,
    kind:char,
    corners: u32,
    size:u32,
    rooted: bool,
}
impl Plant{
    fn add_size(&mut self){
        self.size +=1;
    }
    fn add_corners(&mut self, n:u32){
        self.corners +=n;
    }
    fn rooted(&mut self){
        self.rooted =true;
    }
}

fn inner_corners(plant:&Plant, plant_grid:&Grid<Plant>) -> u32{
    [(-1,-1), (1,-1), (-1,1), (1,1)]
        .iter()
        .map(
            |v|{
                let neighbour_corner = plant_grid.get((plant.x as i32 + v.0) as usize, (plant.y as i32 + v.1) as usize);
                let neighbour1 =  plant_grid.get((plant.x as i32 + v.0) as usize, (plant.y as i32) as usize);
                let neighbour2 =  plant_grid.get((plant.x as i32) as usize, (plant.y as i32 + v.1) as usize);
                if  neighbour_corner.is_some() && neighbour_corner.unwrap().kind != plant.kind &&
                    (neighbour1.is_none() || neighbour1.unwrap().kind == plant.kind) &&
                    (neighbour2.is_none() || neighbour2.unwrap().kind == plant.kind)
                {
                    1
                } else{
                    0
                }
            })
        .sum::<u32>()
}     
fn outer_corners(plant:&Plant, plant_grid:&Grid<Plant>) -> u32{
    [(-1,-1), (1,-1), (-1,1), (1,1)]
        .iter()
        .map(
            |v|{
                // let neighbour_corner = plant_grid.get((plant.x as i32 + v.0) as usize, (plant.y as i32 + v.1) as usize);
                let neighbour1 =  plant_grid.get((plant.x as i32 + v.0) as usize, (plant.y as i32) as usize);
                let neighbour2 =  plant_grid.get((plant.x as i32) as usize, (plant.y as i32 + v.1) as usize);
                if  (neighbour1.is_none() || neighbour1.unwrap().kind != plant.kind) &&
                    (neighbour2.is_none() || neighbour2.unwrap().kind != plant.kind)
                {
                    1
                } else{
                    0
                }
            })
        .sum::<u32>()
}     

fn root_bfs(plant_grid: &mut Grid<Plant>, root:(usize, usize)){
    if plant_grid.get(root.0, root.1).unwrap().rooted {
        return
    }
    let root_kind = plant_grid.get(root.0, root.1).unwrap().kind;
    let mut seen = vec![root];
    let mut stack = vec![root];
    while let Some(pos) = stack.pop(){
        let corners = outer_corners(plant_grid.get(pos.0, pos.1 ).unwrap(), plant_grid) + inner_corners(plant_grid.get(pos.0, pos.1 ).unwrap(), plant_grid);
        plant_grid.get_mut(root.0, root.1).unwrap().add_size();
        plant_grid.get_mut(root.0, root.1).unwrap().add_corners(corners);
        plant_grid.get_mut(pos.0, pos.1).unwrap().rooted();
        for v in [(-1,0), (0,-1), (0,1), (1,0)]{
            let neighbour_pos = ((pos.0 as i32 + v.0) as usize, (pos.1 as i32 + v.1) as usize);
            let neighbour = plant_grid.get(neighbour_pos.0, neighbour_pos.1 );
            if neighbour.is_some() && !seen.contains(&neighbour_pos) && neighbour.unwrap().kind == root_kind{
                stack.push(neighbour_pos);
                seen.push(neighbour_pos);
            }
        }
    }
}

pub fn solution(input: String) -> String { 
    let map = input
        .lines()
        .map(
            |line| 
            line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut plant_grid = Grid::new(
        map
            .iter()
            .enumerate()
            .map(
                |line| 
                line.1  .iter()
                        .enumerate()
                        .map(
                            move |c|
                            Plant {x:c.0, y:line.0, kind:*c.1, size:0, corners:0, rooted:false})
                        .collect::<Vec<Plant>>())
            .collect::<Vec<Vec<Plant>>>());

    let height = plant_grid.map.len();
    let width = plant_grid.map[0].len();
    (0..height).for_each(
        |y|
        (0..width).for_each(
            |x|{
                root_bfs(&mut plant_grid, (x,y));
            }));

    format!("{:?}",plant_grid.map.iter().flatten().map(|p| p.corners * p.size ).sum::<u32>()) 
} 
