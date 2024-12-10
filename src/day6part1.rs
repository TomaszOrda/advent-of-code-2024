

fn rotate_90(dir: (i32,i32)) -> (i32,i32){
    (-dir.1, dir.0) 
}

// Rather imperative code

fn number_of_visited_positions(map: &mut [Vec<char>], guard_position: &mut (usize, usize), guard_direction: &mut (i32,i32)) -> i32{
    let mut result = 1;
    loop{
        if guard_position.0 as i32 + guard_direction.0 <0 || guard_position.1 as i32 + guard_direction.1 < 0{
            break
        }
        let new_guard_position = ((guard_position.0 as i32 + guard_direction.0) as usize,  (guard_position.1 as i32 + guard_direction.1) as usize);
        match map.get(new_guard_position.0).unwrap_or(&vec![]).get(new_guard_position.1).unwrap_or(&'!'){
            '!' => break,
            '#' => {
                let new_guard_direction = rotate_90(*guard_direction);
                guard_direction.0 = new_guard_direction.0;
                guard_direction.1 = new_guard_direction.1;
            },
            '.' => {
                map[guard_position.0][guard_position.1] = 'X';
                guard_position.0 = new_guard_position.0;
                guard_position.1 = new_guard_position.1;
                result +=1;
            },
            'X' =>{
                map[guard_position.0][guard_position.1] = 'X';
                guard_position.0 = new_guard_position.0;
                guard_position.1 = new_guard_position.1;
            }
            _ => panic!("unknown map symbol")
        }
    }
    result
    
}


pub fn solution(input: String) -> String {
    let mut map: Vec<Vec<char>> = input
                                    .clone()
                                    .lines()
                                    .map(|line| line.chars().collect::<Vec<char>>())
                                    .rev()
                                    .collect();
    let guard_position_flat = map.iter().flatten().position(|g| ['^'].contains(g)).unwrap();
    let width = map[0].len();

    format!("{:?}",number_of_visited_positions(&mut map, &mut (guard_position_flat / width, guard_position_flat % width), &mut (1,0)))
}