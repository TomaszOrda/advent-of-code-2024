//This time decided on unoptimal straightforward approach
fn compact(memory:Vec<Option<u64>>) -> Vec<u64>{
    let occupied_cells = memory.iter().filter(|cell| cell.is_some()).count();

    let mut pointer_end = memory.iter().rev().filter(|cell| cell.is_some());
    memory[0..occupied_cells]
        .iter()
        .map(
            |cell|
            if cell.is_some(){
                cell
            }else{
                pointer_end.next().unwrap()
            }.unwrap())
        .collect::<Vec<u64>>()
}

pub fn solution(input: String) -> String {

    let memory = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .flat_map(
            |(id,n)| 
            if id%2 == 0 {
                vec![Some((id as u64 +1 )/2); n as usize]
            }else{
                vec![None; n as usize]
            })
        .collect::<Vec<Option<u64>>>();
    

    format!("{:?}", compact(memory).iter().enumerate().map(|(cell, file_id)| cell as u64 * file_id).sum::<u64>())
}