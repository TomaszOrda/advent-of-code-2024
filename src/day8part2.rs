use std::collections::{HashMap, HashSet};


//To make this functional i would have to create an iterator i suppose. Maybe one day.
fn all_antinodes(antena1: &(usize, usize), antena2: &(usize, usize), height:usize, width: usize) -> Vec<(i32, i32)>{
    let vec = (antena2.0 as i32 - antena1.0 as i32, antena2.1 as i32 - antena1.1 as i32); 
    let mut res = vec![(antena1.0 as i32, antena1.1 as i32), (antena2.0 as i32, antena2.1 as i32)];
    while (0..height as i32).contains(&(res.last().unwrap().0 + vec.0)) && (0..width as i32).contains(&(res.last().unwrap().1 + vec.1)){
        res.push((res.last().unwrap().0 + vec.0, res.last().unwrap().1 + vec.1))
    }
    while (0..height as i32).contains(&(res.first().unwrap().0 + vec.0)) && (0..width as i32).contains(&(res.first().unwrap().1 + vec.1)){
        res.insert(0, (res.first().unwrap().0 + vec.0, res.first().unwrap().1 + vec.1))
    }

    res
}

fn antinodes(frequencies: HashMap<char, Vec<(usize, usize)>>, height:usize, width: usize) -> HashSet<(i32, i32)>{
    frequencies.iter().flat_map(
        |matching_antenas| 
        matching_antenas.1.iter().flat_map(
            |antena1| 
            matching_antenas.1.iter().flat_map(
                move |antena2|
                if antena1 != antena2 {all_antinodes(antena1, antena2, height, width)} else {vec![]}
            ))).collect()

}

pub fn solution(input: String) -> String {
    
    let antenas_map: Vec<Vec<char>> = input
                                    .clone()
                                    .lines()
                                    .map(|line| line.chars().collect::<Vec<char>>())
                                    .collect();
    let (height, width) = (antenas_map.len(), antenas_map[0].len());
    let antenas: Vec<(char, (usize, usize))> = antenas_map.iter()
                                                        .flatten()
                                                        .enumerate()
                                                        .filter(|&(_id, frequency)| frequency!=&'.')
                                                        .map(|(id, frequency)| (*frequency, (id/width,id%width)))
                                                        .collect();
    let mut frequencies: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    antenas.iter().for_each(|antena| frequencies.entry(antena.0).or_default().push(antena.1));

    format!("{:?}",antinodes(frequencies, height, width).len())
    //Note: I do not need a filter anymore
}