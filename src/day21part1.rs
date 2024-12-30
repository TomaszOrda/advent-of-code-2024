use std::{cmp::Ordering, collections::HashMap};
struct Cache{
    memory: HashMap<(char, char, u32), Vec<char>>
}
fn numeric_pad_to_coords(key:char) -> (u32, u32){
    match key {
        'A' => (2,3),
        '0' => (1,3),
        n => {
            let number = n.to_digit(10).unwrap();
            let x = (number - 1) % 3;
            let y = 2 - (number - 1) / 3;
            (x,y)
        }
    }
}
fn directional_pad_to_coords(key:char) -> (u32, u32){
    match key {
        'A' => (2,0),
        '^' => (1,0),
        '<' => (0,1),
        'v' => (1,1),
        '>' => (2,1),
        _ => panic!()
    }
}
fn all_sequences(seq_a: Vec<char>, seq_b: Vec<char>) -> Vec<Vec<char>>{
    if seq_a.is_empty(){
        return vec![seq_b]
    }
    if seq_b.is_empty(){
        return vec![seq_a]
    }
    let mut left: Vec<Vec<char>> = all_sequences(seq_a[1..].to_vec(), seq_b.clone()).iter_mut().map(|v| {v.insert(0, seq_a[0]); v.clone()}).collect();
    let right: Vec<Vec<char>>    = all_sequences(seq_a, seq_b[1..].to_vec()).iter_mut().map(|v| {v.insert(0, seq_b[0]); v.clone()}).collect();
    left.extend(right);
    left
}
fn panic_inducing_sequence(coords_start: (u32, u32), coords_panic: (u32, u32), sequence: &[char] ) -> bool{
    if coords_start == coords_panic{
        return true
    }
    if sequence.is_empty(){
        return false
    }
    match sequence[0]{
        '>' => false,
        '<' => panic_inducing_sequence((coords_start.0-1, coords_start.1  ), coords_panic, &sequence[1..] ),
        'v' => panic_inducing_sequence((coords_start.0  , coords_start.1+1), coords_panic, &sequence[1..] ),
        '^' => panic_inducing_sequence((coords_start.0  , coords_start.1-1), coords_panic, &sequence[1..] ),
        _ => panic!("")
    }
}
fn vailable_sequences(start:char, end:char) -> Vec<Vec<char>>{
    let keypad_arrow = ['v', '<', '^', '>'].contains(&start) || ['v', '<', '^', '>'].contains(&end);
    let coords_start = if keypad_arrow {directional_pad_to_coords(start)} else {numeric_pad_to_coords(start)};
    let coords_end   = if keypad_arrow {directional_pad_to_coords(end  )} else {numeric_pad_to_coords(end  )};
    let coords_panic = if keypad_arrow {(0,0)} else {(0,3)};
    let dir_x = match coords_start.0.cmp(&coords_end.0){
        Ordering::Less => '>',
        _ => '<',
    };
    let dir_y = match coords_start.1.cmp(&coords_end.1){
        Ordering::Less => 'v',
        _ => '^',
    };
    let x_distance = coords_end.0.abs_diff(coords_start.0) as usize;
    let y_distance = coords_end.1.abs_diff(coords_start.1) as usize;
    let result = all_sequences(
        std::iter::repeat(dir_x)
            .take(x_distance)
            .collect(), 
        std::iter::repeat(dir_y)
            .take(y_distance)
            .collect());
    
    result.into_iter().filter(|sequence| !panic_inducing_sequence(coords_start, coords_panic, sequence)).collect()
}

fn to_user_pad_sequence(cache: &Cache, sequence: &[char], phase: u32) -> Vec<char>{
    if phase==3{
        return sequence.to_vec()
    }
    let result = std::iter::once(&'A')
        .chain(sequence.iter())
        .collect::<Vec<&char>>()
        .windows(2)
        .flat_map(
            |keys|
            cache.memory.get(&(*keys[0], *keys[1], phase)).unwrap()
        )
        .copied()
        .collect::<Vec<char>>();
    to_user_pad_sequence(cache, &result, phase+1)
}

fn shortest_robot_sequence(cache: &Cache, from: char, to: char, phase: u32) -> Vec<char>{
    let sequence = vailable_sequences(from, to);
    sequence
        .into_iter()
        .min_by(
            |seq1, seq2| 
            to_user_pad_sequence(cache, seq1, phase+1).len().cmp(&to_user_pad_sequence(cache, seq2, phase+1).len())
        )
        .unwrap()
        .into_iter()
        .chain(std::iter::once('A'))
        .collect::<Vec<char>>()
}

fn complexity(sequence: &[char], code: &[char]) -> u32{
    sequence.len() as u32 * code.iter().collect::<String>().trim_matches('A').parse::<u32>().unwrap()
}

pub fn solution(input: String) -> String { 
    let codes = input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

    let mut cache = Cache {memory: HashMap::new()};

    let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'A'];
    let arrows  = ['v', '<', '^', '>', 'A'];
    for phase in [2,1]{
        for start in arrows{
            for end in arrows{
                let shortest_sequence = shortest_robot_sequence(&cache, start, end, phase);
                cache.memory.insert((start, end, phase), shortest_sequence);
            }
        }
    }
    for phase in [0]{
        for start in digits{
            for end in digits{
                let shortest_sequence = shortest_robot_sequence(&cache, start, end, phase);
                cache.memory.insert((start, end, phase), shortest_sequence);
            }
        }
    }

    format!("{:?}",codes.iter().map(|code| complexity(&to_user_pad_sequence(&cache, code, 0), code)).sum::<u32>())
} 
#[test]
fn basic_test() {
    let input = "029A
                        980A
                        179A
                        456A
                        379A".chars().filter(|&c| c!=' ').collect::<String>();
    // let input = "379A".chars().filter(|&c| c!=' ').collect::<String>();
    assert_eq!(solution(input).parse::<u32>().unwrap(), 126384)
}
