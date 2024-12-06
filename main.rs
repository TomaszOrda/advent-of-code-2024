macro_rules! mod_day_and_task{
    ($name:ident) =>{
        mod $name ;
        use $name::main as solution;
    }
}

mod_day_and_task!(day6part2);
fn main(){
    solution()
}
