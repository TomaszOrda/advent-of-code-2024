#[derive(Debug, Clone, Copy)]
enum Direction{
    N,
    E,
    S,
    W
}
impl Direction{
    fn to_vector(self) -> (i32, i32){
        match self{
            Self::N => (-1,  0),
            Self::E => ( 0,  1),
            Self::S => ( 1,  0),
            Self::W => ( 0, -1),
        }
    }
    fn to_layer(self) -> usize{
        match self{
            Self::N => 0,
            Self::E => 1,
            Self::S => 2,
            Self::W => 3,
        }
    }
    fn apply(&self, pos: (usize, usize)) -> (usize, usize){
        let v = self.to_vector();
        ((pos.0 as i32 + v.0) as usize,
         (pos.1 as i32 + v.1) as usize)
    }
    fn rotate_90(&self) -> Vec<Direction>{
        match self{
            Direction::N | Direction::S => vec![Direction::E, Direction::W],
            Direction::E | Direction::W => vec![Direction::N, Direction::S]
        }
    }

}
fn directions_with_cost(direction: Direction) -> Vec<(Direction, u32)>{
    direction.rotate_90().iter().map(|&dir| (dir, 1000)).chain(std::iter::once((direction, 1))).collect::<Vec<(Direction, u32)>>()
    //rotating 180 is always suboptimal
    //slight difference because of how representation changed
}

fn is_road(map:&[Vec<char>], new_pos: (usize, usize)) -> bool{
    map[new_pos.0][new_pos.1] == '.' || map[new_pos.0][new_pos.1] == 'S' || map[new_pos.0][new_pos.1] == 'E'
}
//changed the paradigm slightly
fn best_scores(map:&[Vec<char>], start: (usize, usize), start_dir: Direction) -> Vec<Vec<Vec<u32>>>{
    let mut best_partial_scores = vec![vec![vec![u32::MAX;4]; map[0].len()]; map.len()]; 
    best_partial_scores[start.0][start.1][start_dir.to_layer()]=0;
    let mut stack = vec![((start.0, start.1), start_dir, 0)];
    while let Some(state) = stack.pop(){
        let pos = state.0;
        let dir = state.1;
        let score = state.2;

        for new_dir_cost in directions_with_cost(dir){
            let new_dir = new_dir_cost.0;
            let new_score = score + new_dir_cost.1;
            if new_score< best_partial_scores[pos.0][pos.1][new_dir.to_layer()]{
                best_partial_scores[pos.0][pos.1][new_dir.to_layer()] = new_score;
                stack.insert(0, (pos,new_dir, new_score)); 
            }
        }

        let new_pos = dir.apply(pos);
        let new_score = score + 1;
        if is_road(map, new_pos) && new_score < best_partial_scores[new_pos.0][new_pos.1][dir.to_layer()]{
            best_partial_scores[new_pos.0][new_pos.1][dir.to_layer()] = new_score;
            stack.insert(0, (new_pos,dir, new_score));
        }
    }
    best_partial_scores
}


pub fn solution(input: String) -> String { 
    let map = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let start = {
        let start_flat = map.iter().flatten().position(|&c| c == 'S').unwrap();
        (start_flat / map[0].len(), start_flat % map[0].len())
    };
    let end = {
        let end_flat = map.iter().flatten().position(|&c| c == 'E').unwrap();
        (end_flat / map[0].len(), end_flat % map[0].len())
    };
    let scores_forward  = best_scores(&map, start, Direction::E);
    let scores_backward = best_scores(&map, end, Direction::S);
    let optimal_path_cost = *scores_forward[end.0][end.1].iter().min().unwrap();
    format!("{:?}",  
        scores_forward
            .iter()
            .zip(scores_backward.iter())
            .flat_map(
                |(line_forward, line_backward)| 
                line_forward
                    .iter()
                    .zip(line_backward.iter())
                    .filter(
                        |(sf, sb)|
                        sf[0]<u32::MAX && 
                        (sf[0] + sb[2] == optimal_path_cost ||
                         sf[1] + sb[3] == optimal_path_cost ||
                         sf[2] + sb[0] == optimal_path_cost ||
                         sf[3] + sb[1] == optimal_path_cost   )))
            .count())
} 