use regex::Regex;

#[derive(Debug)]
struct Robot{
    v: (usize, usize),
    p: (usize, usize)
}

impl Robot{
    fn step(&mut self, width:usize, height:usize){
        //thanks to the v being positive we do not need to worry about wrapping around the left edge
        self.p.0 += self.v.0;
        self.p.0 %= width;
        self.p.1 += self.v.1;
        self.p.1 %= height;
        
    }
}
fn trim(word:&str) -> &str{
    word.trim_start_matches(['p', 'v', '='])
}

fn print_grid(grid: &str, width:usize){
    let chars = grid.chars().collect::<Vec<char>>();
    let lines = chars.chunks(width);
    for line in lines{
        println!("{}", line.iter().collect::<String>());
    }
}
fn contains_tree(grid: &str, width:usize) -> bool{
    let re = Regex::new(&format!(r"#.{{{}}}###.{{{}}}#####", width-2, width-4)).unwrap(); 
    re.is_match(&grid)
}

pub fn solution(input: String) -> String { 
    let (width, height) = if input.len() < 1000 {(11, 7)} else {(101, 103)}; //Test and input differs in the grid size! Change width and height here accordingly.
    
    let mut robots = input
        .lines()
        .map(
            |line| 
            line.split_whitespace()
                .map(
                    |w| 
                    trim(w).split(",")
                                 .map(|x| x.parse::<i32>().unwrap())
                                 .collect::<Vec<i32>>())
                .collect::<Vec<Vec<i32>>>())
        .map(
            |line|
                                                                
            Robot {
                p: (line[0][0] as usize, 
                    line[0][1] as usize), 
                v: ((((line[1][0] + width  as i32) % width  as i32) as usize), //translate v into positive values
                    (((line[1][1] + height as i32) % height as i32) as usize))
                
            })
        .collect::<Vec<Robot>>();
    //changed how the grid is rendered, but it didnt help
    let mut grid = vec!['.'; height*width].iter().collect::<String>();
    
    for i in 1..100000{
        //there might be overlaps in the image perhaps?
        robots.iter().for_each(|robot| grid.replace_range((robot.p.1 * width + robot.p.0)..=(robot.p.1 * width + robot.p.0), "."));
        robots.iter_mut().for_each(|robot| robot.step(width, height));
        robots.iter().for_each(|robot| grid.replace_range((robot.p.1 * width + robot.p.0)..=(robot.p.1 * width + robot.p.0), "#"));
        if contains_tree(&grid, width){
            print_grid(&grid, width);
            return format!("{:?}", i)
        }
    }
    format!("{:?}","Not found")
} 
