//This one was tedious
//However i did get around implementing my own grid (which was easier than i thought), maybe ill export it as an util (that is if i need it for a future task)
//It was an exercise in writing big libraries I feel like. Good experience to have; Ungrateful labor at the same time.
//I could've used grid/mattrix and bfs from external crate. Maybe i will next time.
// I think i whould rewrite it this way:
// using scanning or bfs have each plant in a plot remmeber whole plots root plant, then when counting corners accumulate the value in the root
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
}
#[derive(PartialEq)]
struct Plant{
    x:usize,
    y:usize,
    kind:char,
    perimeter:u32
}

fn bfs(root_plant:&Plant, plant_grid:&Grid<Plant>, corners: &Grid<u32>, values: &mut [Vec<u32>]){
    let mut seen = vec![root_plant];
    let mut stack = vec![root_plant];
    let mut value = 0;
    while let Some(plant) = stack.pop(){
        if values[plant.y][plant.x] != 0{
            value = values[plant.y][plant.x];
            break;
        }
        value += corners.get(plant.x, plant.y).unwrap();
        for v in [(-1,0), (0,-1), (0,1), (1,0)]{
            let neighbour = plant_grid.get((plant.x as i32 + v.0) as usize, (plant.y as i32 + v.1) as usize);
            if let Some(neighbour_unwrap) = neighbour{
                if  !seen.contains(&neighbour.unwrap()) && neighbour.unwrap().kind == plant.kind{
                    stack.push(neighbour_unwrap);
                    seen.push(neighbour_unwrap);
                }
            }
        }
    }
    values[root_plant.y][root_plant.x] = value;
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

fn find_corners(plant_grid:&Grid<Plant>) -> Vec<Vec<u32>>{
    plant_grid.map
        .iter()
        .map(
            |line|
            line.iter()
                .map( 
                    |plant|
                    inner_corners(plant, plant_grid) + outer_corners(plant, plant_grid))
                .collect::<Vec<u32>>()
        )
        .collect::<Vec<Vec<u32>>>()
}

pub fn solution(input: String) -> String { 
    let map = input
        .lines()
        .map(
            |line| 
            line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let plant_grid = Grid::new(
        map
            .iter()
            .enumerate()
            .map(
                |line| 
                line.1  .iter()
                        .enumerate()
                        .map(
                            move |c|
                            Plant {x:c.0, y:line.0, kind:*c.1, perimeter:0})
                        .collect::<Vec<Plant>>())
            .collect::<Vec<Vec<Plant>>>());
    
    let corners = Grid::new(find_corners(&plant_grid));

    let mut values = vec![vec![0; map[0].len()]; map.len()];
    plant_grid.map.iter().flatten().for_each(|plant| bfs(plant, &plant_grid, &corners, &mut values));

    format!("{:?}",values.iter().flatten().sum::<u32>()) 
} 
