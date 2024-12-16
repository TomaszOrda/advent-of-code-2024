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
    direction.rotate_90().iter().map(|&dir| (dir, 1001)).chain(std::iter::once((direction, 1))).collect::<Vec<(Direction, u32)>>()
    //rotating 180 is always suboptimal
}

fn is_road(map:&[Vec<char>], new_pos: (usize, usize)) -> bool{
    map[new_pos.0][new_pos.1] == '.' || map[new_pos.0][new_pos.1] == 'S' || map[new_pos.0][new_pos.1] == 'E'
}

fn best_score(map:&[Vec<char>], start: (usize, usize), end: (usize, usize)) -> u32{
    let mut best_partial_scores = vec![vec![u32::MAX;map[0].len()]; map.len()]; //is this a good practice? I suppose, considering that there are no overflows in rust.
    let mut stack = vec![(start, Direction::E,0)];
    while let Some(state) = stack.pop(){
        let pos = state.0;
        let dir = state.1;
        let score = state.2;
        if score < best_partial_scores[pos.0][pos.1]{
            best_partial_scores[pos.0][pos.1] = score;
            if pos == end{
                continue
            }
            for new_dir_cost in directions_with_cost(dir){
                let new_dir = new_dir_cost.0;
                let new_pos = new_dir_cost.0.apply(pos);
                            let new_score = score + new_dir_cost.1;
                if is_road(map, new_pos){
                        stack.insert(0, (new_pos,new_dir, new_score)); //This ones faster, bfs considers the unoptimal paths less
                        // stack.push((new_pos,new_dir, new_score));
                }
            }
        }

        
    }
    best_partial_scores[end.0][end.1]
}

pub fn solution(input: String) -> String { 
    let map = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let start = {
        let start_flat = map.iter().flatten().position(|&c| c == 'S').unwrap();
        (start_flat / map[0].len(), start_flat % map[0].len())
    };//I think thats a cool way of doing something like that. It does take twice as amny lines however. But is very readable.
    let end = {
        let end_flat = map.iter().flatten().position(|&c| c == 'E').unwrap();
        (end_flat / map[0].len(), end_flat % map[0].len())
    };

    format!("{:?}",best_score(&map, start, end)) 
} 
