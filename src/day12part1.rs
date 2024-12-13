//I could revise this solutions but.. it would look just like day12part2r. Thus such cahnge is unnecessary
//and this solution is not as bad
#[derive(Debug, Eq, PartialEq, Clone)]
struct Plant{
    x:usize,
    y:usize,
    kind:char,
    perimeter:u32
}

fn perimieter_bfs(root_plant:&Plant, plant_map:&[Vec<Plant>], known_perimeters: &mut [Vec<u32>]) -> u32{
    let mempty = vec![];
    let plant_clone = root_plant.clone();
    let mut plot = vec![&plant_clone];
    let mut stack = vec![&plant_clone];
    let mut perimeter = 0;
    while let Some(plant) = stack.pop(){
        let mut local_perimeter = 4;
        for v in [(-1,0), (0,-1), (0,1), (1,0)]{
                if let Some(neighbour) = plant_map.get((plant.y as i32 + v.1) as usize).unwrap_or(&mempty).get((plant.x as i32 + v.0) as usize){
                    if neighbour.kind == plant.kind{
                        if known_perimeters[neighbour.y][neighbour.x]>0{
                            known_perimeters[root_plant.y][root_plant.x]= known_perimeters[neighbour.y][neighbour.x];
                            return known_perimeters[root_plant.y][root_plant.x]
                        }
                        local_perimeter-=1;
                        if !plot.contains(&neighbour){
                            stack.push(neighbour);
                            plot.push(neighbour);
                        }
                    }
                }
            };
        perimeter += local_perimeter;
    }
    known_perimeters[root_plant.y][root_plant.x] = perimeter;
    perimeter
}

pub fn solution(input: String) -> String { 
    let map = input
        .lines()
        .map(
            |line| 
            line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let plant_map = map
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
        .collect::<Vec<Vec<Plant>>>();
    let mut known_perimeters = vec![vec![0; map[0].len()]; map.len()];

    format!("{:?}",plant_map.iter().flatten().map(|plant| perimieter_bfs(plant, &plant_map, &mut known_perimeters)).sum::<u32>()) 
} 
