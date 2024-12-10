use std::collections::{HashMap, HashSet};

//More functional than the previous one, but doing basically the same

fn antinode(antena1: &(usize, usize), antena2: &(usize, usize)) -> (i32, i32){
    let vec = (antena2.0 as i32 - antena1.0 as i32, antena2.1 as i32 - antena1.1 as i32); 
    (antena2.0 as i32 + vec.0, antena2.1 as i32 + vec.1) //this could be shorter but at the slight cost of readability
}

fn antinodes(frequencies: HashMap<char, Vec<(usize, usize)>>) -> HashSet<(i32, i32)>{
    frequencies.iter().flat_map(
        |matching_antenas| 
        matching_antenas.1.iter().flat_map(
            |antena1| 
            matching_antenas.1.iter().flat_map(
                move |antena2|
                if antena1 != antena2 {vec![antinode(antena1, antena2), antinode(antena2, antena1)]} else {vec![]}
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

    format!("{:?}",antinodes(frequencies).iter().filter(|&&(row, col)| row>=0 && col>=0 && row<height as i32 && col<height as i32).count() )
}