

//This time i keep the data in the original form. It is not a pretty solution. I should rewritre it Some(day)... maybe...
fn compact(memory:&mut Vec<(Option<u64>, u64)>,  end_pointer:usize){
    let mut end_pointer = end_pointer;
    while end_pointer > 0{
      
        if memory[end_pointer].0.is_some(){
            let free_space = memory.iter().enumerate().position(|(pos,sector)| sector.0.is_none() && sector.1 >= memory[end_pointer].1 && pos < end_pointer);
            if free_space.is_some() {
                let pos = free_space.unwrap();
                memory[pos].1 -= memory[end_pointer].1;
                memory.insert(pos, memory[end_pointer]);
                memory.insert(pos, (None,0));
                memory[end_pointer+2] = (None,memory[pos+1].1);
                end_pointer+=2;
            }
        }
        end_pointer -=1
    }
}

pub fn solution(input: String) -> String {

    let mut memory = input
        .chars()
        .enumerate()
        .map(|(id,c)| (if id%2 == 0 {Some(id as u64/2)} else {None}, c.to_digit(10).unwrap() as u64))
        .collect::<Vec<(Option<u64>, u64)>>();
    let end_pointer = memory.len()-1;
    compact(&mut memory, end_pointer);
    format!("{:?}", memory.iter().flat_map(|(id, size)| match id { Some(file_id) => vec![file_id; *size as usize], None => vec![&0; *size as usize]}).enumerate().map(|(cell, file_id)| cell as u64 * file_id).sum::<u64>())
}