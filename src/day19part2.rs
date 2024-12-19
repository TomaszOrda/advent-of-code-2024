#[derive(Debug, Clone)]
struct Slice{
    length: usize
}
struct SlicedDesign{
    slices: Vec<Vec<Slice>>
}

fn all_matches(input:&str, pat:&str) -> Vec<usize>{
    (0..input.len()).filter(|id| input.split_at(*id).1.starts_with(pat)).collect::<Vec<usize>>()
}

fn design_to_slices(design: &str, towels: &[&str])-> Vec<Vec<Slice>>{
    let mut res = vec![vec![]; design.len()];
    for towel in towels{
        //match indices can only find non-overlaping matches!
        // for (id, _) in design.match_indices(towel){
        for id in all_matches(design, towel){
            res[id].push(Slice {length:towel.len()})
        }
        // }
    }
    res
}

fn possible_arrangements(sliced_design: &[Vec<Slice>]) -> u64{
    let mut suffix_sum: Vec<u64> = vec![0; sliced_design.len()];
    for id in (0..sliced_design.len()).rev(){
        suffix_sum[id] = sliced_design[id]
            .iter()
            .map(
                |slice| 
                if id + slice.length == sliced_design.len() {1} else {suffix_sum[id + slice.length]})
            .sum::<u64>()
    }
    suffix_sum[0]
}

pub fn solution(input: String) -> String { 
    let towels = input
            .lines()
            .next()
            .unwrap()
            .split(", ")
            .collect::<Vec<&str>>();
    
    
    let designs = input
        .lines()
        .skip(2)
        .collect::<Vec<&str>>();

    let sliced_designs = designs
        .iter()
        .map(
            |design|
            SlicedDesign {slices: design_to_slices(design, &towels)})
        .collect::<Vec<SlicedDesign>>();

    format!("{:?}",sliced_designs.iter().map(|sliced_design| possible_arrangements(&sliced_design.slices)).sum::<u64>()) 
} 
