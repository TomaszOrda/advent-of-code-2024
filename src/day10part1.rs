use std::collections::HashSet;
#[derive(PartialEq, Eq, Hash)]
struct Position{
    x:usize,
    y:usize,
    height:u32
}

fn score(map:&[Vec<Position>], trailhead:&Position) -> usize{
    let mut trailends = vec![];

    let mut stack= vec![trailhead];
    while let Some(trail) = stack.pop(){
        if trail.height == 9{
            trailends.push(trail);
        }
        for step in  [(-1,0), (0,-1), (0,1), (1,0), ]{
            let nextx = step.0 +trail.x as i8;
            let nexty = step.1 +trail.y as i8;
            if (0..map.len() as i8).contains(&nexty) && (0..map[0].len() as i8).contains( &nextx ) {
                let nextposition = &map[nexty as usize][nextx as usize];
                if nextposition.height == trail.height+1{
                    stack.push(nextposition)
                }
            } 
        }
    }
    trailends.iter().collect::<HashSet<&&Position>>().len()
}

pub fn solution(input: String) -> String {
    let map = input
        .lines()
        .enumerate()
        .map(
            |line| 
            line.1.chars()
                  .enumerate()
                  .map(
                    |c| 
                    Position{
                        x: c.0, 
                        y: line.0, 
                        height: c.1.to_digit(10).unwrap()
                    })
                  .collect::<Vec<Position>>())
        .collect::<Vec<Vec<Position>>>();
    
    let trailheads = map.iter().flatten().filter(|p| p.height==0);

    format!("{:?}",trailheads.map(|th| score(&map, th) as u32).sum::<u32>())
    // format!("{:?}",trailheads.map(|th| score(&map, th) as u32).collect::<Vec<u32>>())
}