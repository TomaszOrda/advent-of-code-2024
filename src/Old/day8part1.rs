use std::collections::{HashMap, HashSet};


//imperative code once again
fn antinode(antena1: &(usize, usize), antena2: &(usize, usize)) -> (i32, i32){
    let vec = (antena2.0 as i32 - antena1.0 as i32, antena2.1 as i32 - antena1.1 as i32); 
    (antena2.0 as i32 + vec.0, antena2.1 as i32 + vec.1) //this could be shorter but at the slight cost of readability
}

fn antinodes(frequencies: HashMap<char, Vec<(usize, usize)>>) -> HashSet<(i32, i32)>{
    let mut antinodes: Vec<(i32, i32)> = vec![];
    for frequency in frequencies.keys(){
        let matching_antenas = frequencies.get(frequency).unwrap();
        for antena1 in matching_antenas{
            for antena2 in matching_antenas{
                if antena1!=antena2{
                    antinodes.push(antinode(antena1, antena2));
                    antinodes.push(antinode(antena2, antena1));
                }
            }
        }
    }

    antinodes.into_iter().collect()
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