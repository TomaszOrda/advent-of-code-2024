

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

fn correct_an_update(rules: &[(i32, i32)], update: Vec<i32>) -> Vec<i32>{
    let mut corrected_update = update;
    rules
        .iter()
        .for_each(
            |r|{
                let pos1 = corrected_update.iter().position(|&x| x==r.0).unwrap_or(0);
                let pos2 = corrected_update.iter().position(|&x| x==r.1).unwrap_or(corrected_update.len());
                if  pos1 > pos2 {
                    corrected_update.swap(pos1, pos2);
                }
            });
    if correctly_ordered_update(rules, &corrected_update){
        corrected_update
    }else {
        correct_an_update(rules, corrected_update)
    }
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
    
    let アップデート: Vec<Vec<i32>> = lines
                                    .iter()
                                    .filter(|s| s.contains(','))
                                    .map(|s| s.split(','))
                                    .map(|v| v.map(|c| c.parse::<i32>().unwrap()).collect::<Vec<i32>>())
                                    .collect();

    format!("{:?}",アップデート.into_iter().filter(|u| !correctly_ordered_update(&rules, u)).map(|u| correct_an_update(&rules, u)).map(|u| middle_page_number(&u)).sum::<i32>() )
}
