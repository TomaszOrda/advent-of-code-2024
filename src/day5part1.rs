

fn correctly_ordered_update(rules: &[(i32, i32)], update: &[i32]) -> bool{
    rules
        .iter()
        .map(
            |r|
            update.iter().position(|&x| x==r.0).unwrap_or(0) 
            <=
            update.iter().position(|&x| x==r.1).unwrap_or(update.len()))
        .all(|b| b)
}

fn middle_page_number(update: &[i32]) -> i32{
    let middle = update.len()/2;
    update[middle]
}


pub fn solution(input: String) -> String {
    let lines: Vec<String> = input
                                    .clone()
                                    .lines()
                                    .map(|line| line.to_string())
                                    .collect();

    let rules: Vec<(i32, i32)> = lines
                                    .iter()
                                    .filter(|s| s.contains('|'))
                                    .map(|s| s.split('|'))
                                    .map(|mut v| ( v.next().unwrap().parse::<i32>().unwrap(), 
                                                                v.next().unwrap().parse::<i32>().unwrap()))
                                    .collect();
    
    let updates: Vec<Vec<i32>> = lines
                                    .iter()
                                    .filter(|s| s.contains(','))
                                    .map(|s| s.split(','))
                                    .map(|v| v.map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>())
                                    .collect();

    format!("{:?}",updates.iter().filter(|u| correctly_ordered_update(&rules, u)).map(|u| middle_page_number(u)).sum::<i32>() )
}
