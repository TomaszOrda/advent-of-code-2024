

fn count_xmas_on_id(stream:&[Vec<char>], id:(i32, i32)) -> u32{
    let vectors: Vec<(i32,i32)> = vec![(-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1)];
    vectors.iter()
            .filter(
                |&v| 
                stream.get((id.0+1*v.0) as usize).unwrap_or(&vec![]).get((id.1+1*v.1) as usize).unwrap_or(&'!') == &'M' &&
                stream.get((id.0+2*v.0) as usize).unwrap_or(&vec![]).get((id.1+2*v.1) as usize).unwrap_or(&'!') == &'A' &&
                stream.get((id.0+3*v.0) as usize).unwrap_or(&vec![]).get((id.1+3*v.1) as usize).unwrap_or(&'!') == &'S')
            .count() as u32
}

fn count_xmas(stream:Vec<Vec<char>>) -> u32{
    let xs: Vec<(usize,usize)> = stream.iter()
                                        .enumerate()
                                        .flat_map(
                                            |(id0, v)| 
                                            v.iter()
                                            .collect::<String>()
                                            .char_indices()
                                            .filter(|&(_id, c)| c=='X')
                                            .map(|(id, _c)| id)
                                            .map(|id1| (id0, id1))
                                            .collect::<Vec<(usize,usize)>>())
                                        .collect();
    xs.iter().map(|&id| count_xmas_on_id(&stream, (id.0 as i32, id.1 as i32))).sum()
}

pub fn solution(input: String) -> String {
    let lines: Vec<Vec<char>> = input
                                    .lines()
                                    .map(|s| s.chars()
                                                    .collect::<Vec<char>>())
                                    .collect();

    format!("{:?}",count_xmas(lines))
}
