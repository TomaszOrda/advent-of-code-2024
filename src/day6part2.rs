

fn rotate_90(dir: (i32,i32)) -> (i32,i32){
    (-dir.1, dir.0) 
}

// Rather imperative code

fn guards_path(map: &mut [Vec<char>], guard_position: &mut (usize, usize), guard_direction: &mut (i32,i32)) -> Vec<Vec<char>>{
    loop{
        if guard_position.0 as i32 + guard_direction.0 <0 || guard_position.1 as i32 + guard_direction.1 < 0{
            map[guard_position.0][guard_position.1] = 'X';
            return map.to_owned()
        }
        let new_guard_position = ((guard_position.0 as i32 + guard_direction.0) as usize,  (guard_position.1 as i32 + guard_direction.1) as usize);
        match map.get(new_guard_position.0).unwrap_or(&vec![]).get(new_guard_position.1).unwrap_or(&'!'){
            '!' => {
                map[guard_position.0][guard_position.1] = 'X';
                return map.to_owned()
            },
            '#' => {
                let new_guard_direction = rotate_90(*guard_direction);
                guard_direction.0 = new_guard_direction.0;
                guard_direction.1 = new_guard_direction.1;
            },
            '.' => {
                map[guard_position.0][guard_position.1] = 'X';
                guard_position.0 = new_guard_position.0;
                guard_position.1 = new_guard_position.1;
            },
            'X' =>{
                map[guard_position.0][guard_position.1] = 'X';
                guard_position.0 = new_guard_position.0;
                guard_position.1 = new_guard_position.1;
            }
            _ => panic!("unknown map symbol")
        }
    }
}


fn is_a_loop(map: &mut [Vec<char>], guard_position: &mut (usize, usize), guard_direction: &mut (i32,i32), obstacle: (usize, usize)) -> bool{
    let mut visited_positions: Vec<((usize, usize), (i32, i32))> = vec![];
    map[obstacle.0][obstacle.1] = '#';
    loop{
        if guard_position.0 as i32 + guard_direction.0 <0 || guard_position.1 as i32 + guard_direction.1 < 0{
            break
        }
        let new_guard_position = ((guard_position.0 as i32 + guard_direction.0) as usize,  (guard_position.1 as i32 + guard_direction.1) as usize);
        match map.get(new_guard_position.0).unwrap_or(&vec![]).get(new_guard_position.1).unwrap_or(&'!'){
            '.' | 'X' | '^' => {
                guard_position.0 = new_guard_position.0;
                guard_position.1 = new_guard_position.1;
            }
            '#' => {
                let record = (new_guard_position, *guard_direction);
                if visited_positions.contains(&record){
                    return true
                }
                visited_positions.push(record);
                let new_guard_direction = rotate_90(*guard_direction);
                guard_direction.0 = new_guard_direction.0;
                guard_direction.1 = new_guard_direction.1;
            }
            '!' => break,
            _ => panic!("unknown map symbol")
        }
    }
    false
}

pub fn solution(input: String) -> String {
    let map: Vec<Vec<char>> = input
                                    .clone()
                                    .lines()
                                    .map(|line| line.chars().collect::<Vec<char>>())
                                    .rev()
                                    .collect();
    let guard_position_flat = map.iter().flatten().position(|g| ['^'].contains(g)).unwrap();
    let width = map[0].len();

    let guards_path = guards_path(&mut map.clone(), 
                                                  &mut (guard_position_flat / width, guard_position_flat % width), 
                                                  &mut (1,0));
    let xs = guards_path.iter()
                    .enumerate()
                    .flat_map(
                        |(y,line)| 
                        line.iter()
                            .enumerate()
                            .filter_map(move |(x, c)| if c==&'X' {Some((y,x))} else {None}) )
                    .collect::<Vec<(usize, usize)>>();

    format!("{:?}",xs.iter()
                    .filter(
                        |x| 
                        is_a_loop(&mut map.clone(), &mut (guard_position_flat / width, guard_position_flat % width), &mut (1,0), **x))
                    .count())
}