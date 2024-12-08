macro_rules! mod_day_and_task{
    ($name:ident) =>{
        mod $name ;
        use $name::solution as solution;
    }
}

mod_day_and_task!(day7part2r);
fn main(){
    let args = &mut std::env::args();
    args.next();
    let input_file: String = args.next().unwrap_or_else(|| "No input file provided".to_string());
    let input: String = std::fs::read_to_string(input_file).unwrap();
// test all solutions
// send to github
// Create a template
    println!("{}", solution(input));
}
