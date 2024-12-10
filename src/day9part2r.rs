
//This runs definietly faster, and is prettier

#[derive(Clone, Debug)]
enum MemorySector{
    Empty {size:u64},
    File {id:u64, size:u64},
}
impl MemorySector{
    fn reduce(&mut self, n:u64){
        match &self{
            Self::Empty { size } => *self = MemorySector::Empty{size:size-n},
            _ => panic!("tried to reduce filesize")
        }
    }
}

fn compact(memory:&mut Vec<MemorySector>,  end_pointer:usize){
    let mut end_pointer = end_pointer;
    while end_pointer > 0{
      
        if let  MemorySector::File { id, size: file_size } = memory[end_pointer]{ 
                let free_space = 
                    memory
                        .iter()
                        .enumerate()
                        .position(
                            |(pos,sector)| 
                            match sector { 
                                MemorySector::Empty { size } => size >= &file_size && pos < end_pointer,
                                _=> false
                            });
                if free_space.is_some() {
                    let pos = free_space.unwrap();
                    memory[pos].reduce(file_size);
                    memory.insert(pos, MemorySector::File { id, size: file_size });
                    // memory.insert(pos, MemorySector::Empty { size: 0 });
                    // memory[end_pointer+1] = MemorySector::Empty { size: file_size };
                    memory.remove(end_pointer+1);
                    // end_pointer+=1;
                }
        }
        end_pointer -=1
    }
}

fn sum_a_to_b(a:u64, b:u64)->u64{
    (a..b).sum()
}

fn checksum(memory:Vec<MemorySector>) -> u64{
    let mut position = 0;
    memory
        .iter()
        .map(
            |sector| 
            match sector {
                MemorySector::Empty { size } => {
                    position += size;
                    0
                },
                MemorySector::File { id, size } => {
                    position += size;
                    sum_a_to_b(position-size, position) * id      
                }
            })
        .sum::<u64>()

}

pub fn solution(input: String) -> String {

    let mut memory = input
        .chars()
        .enumerate()
        .map(|(id,size)| if id%2 == 0 {MemorySector::File {id:id as u64 / 2, size:size.to_digit(10).unwrap() as u64}} else {MemorySector::Empty { size:size.to_digit(10).unwrap() as u64 }})
        .collect::<Vec<MemorySector>>();
    let end_pointer = memory.len()-1;
    compact(&mut memory, end_pointer);
    format!("{:?}", checksum(memory))
}