//Same as before, straightforward approach, but this time using enums
//The change of paradigm seems to be the point of this days tasks
#[derive(Debug, Clone, PartialEq, Copy)]
enum MemorySector{
    File(u64, u64),
    Empty(u64)
}
impl MemorySector{
    fn fits(&self, size: u64) -> bool {
        match &self{
            Self::File(_,_) => false,
            Self::Empty(space) => space>= &size
        }
    }
    fn size(&self) -> u64{
        match &self{
            Self::File(_,space) => *space,
            Self::Empty(space) => *space
        }
    }
    fn take_away(&mut self, size: u64){
        match self{
            Self::Empty(ref mut space) => *space-=size,
            Self::File(_, space)  => *self = Self::Empty(*space) 
        }
    }
}

fn compact(memory:&mut Vec<MemorySector>) -> Vec<MemorySector>{
    let mut compacted_memory = memory;
    let mut result = vec![];
    for sector in compacted_memory.iter(){
        match sector{
            MemorySector::Empty(size) =>{
                let mut size = *size;
                loop{
                    match compacted_memory.iter().rposition(|ms| !ms.fits(0) && ms.size()<=size){
                        Some(fms) => {
                            result.push(compacted_memory[fms]);
                            size -= compacted_memory[fms].size();
                            compacted_memory.remove(fms);
                        }
                        None => break
                    }
                }
                result.push(MemorySector::Empty(size));
                // result.extend(res.iter());
            }
            file => result.push(*file)
    }
}
    vec![]
}
    // memory
    //     .iter()
    //     .flat_map(
    //         |sector| 
           
    //         })
    //         .enumerate()
    //         .filter(
    //             |(pos, ms)| 
    //             !memory[..*pos].iter().any(|ems| ems == ms))
    //         .map(
    //             |(_, ms)| 
    //             ms)
    //         .collect()
// }
    // let mut compacted_memory = memory.clone();
    // for i in compacted_memory.len()..0{
    //     let sector = compacted_memory[i];
    //     if sector.fits(0){
    //         continue
    //     }
    //     match compacted_memory.iter().position(|s| match s {
    //         MemorySector::File(_,_) => false,
    //         MemorySector::Empty(size) =>{
    //             size>sector.size()
    //         }}){
    //             Some(pos) => {
                    
    //             }
    //         }
            
        // }.fits(sector.size())){
        //     Some(pos) => if pos<sector_position {
        //         compacted_memory[pos].take_away(sector.size());
        //         compacted_memory.insert(pos, sector.clone());
        //     }
        //     _ => ()
        // }
//     }
//     for (sector_position, sector) in memory.iter().enumerate().filter(|(_,s)| !s.fits(1)).rev(){
        
//     }
//     compacted_memory
//         .iter()
//         .enumerate()
//         .map(
//             |(pos, ms)| 
//             if ! (ms.fits(0) || pos == compacted_memory.iter().position(|msp| msp == ms).unwrap() ){
//                 (pos,MemorySector::Empty(ms.size()))
//             }else{
//                 (pos, ms.clone())
//             })
//         .map(
//             |(_, ms)| 
//             ms)
//         .collect()
// }

fn checksum(memory:Vec<MemorySector>) -> u64{
    let mut position = 0;
    memory.iter().map(|ms| match ms{
        MemorySector::Empty(size) => {
            position += size;
            0
        }
        MemorySector::File(id, size) => {
            id * (position..(position+size)).sum::<u64>()
        }
    }).sum()
}

pub fn solution(input: String) -> String {

    let memory = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(
            |(id,n)| 
            if id%2 == 0 {
                MemorySector::File((id as u64 +1 )/2, n as u64)
            }else{
                MemorySector::Empty(n as u64)
            })
        .collect::<Vec<MemorySector>>();
    

    // format!("{:?}", checksum(compact(memory)))
    format!("{:?}", compact(memory))
}