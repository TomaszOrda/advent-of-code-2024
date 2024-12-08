use std::ops::Neg;


fn looks_like_decreasing_report(report: &[i32]) -> bool{
    [report[0]>report[1], report[1]>report[2], report[2]>report[3]].iter().filter(|&x| *x).count()>1
}

fn is_safe(report: &Vec<i32>, id: usize, dampener_used_up:bool) -> bool{
    if report.len()<id+2{
        return true
    }
    let difference = report[id+1] - report[id];
    if (1..=3).contains(&difference){//difference >= 1 && difference <= 3 {
        return is_safe(report, id+1, dampener_used_up)
    }
    if dampener_used_up{
        return false
    }
    let mut dampened_report_1 = report.clone();
    dampened_report_1.remove(id);
    let mut dampened_report_2 = report.clone();
    dampened_report_2.remove(id+1);
    is_safe(&dampened_report_1, 0, true) 
    ||
    is_safe(&dampened_report_2, 0, true) 
}

fn number_of_safe_reports(reports:Vec<Vec<i32>>) ->i32{
    reports.iter()
        .map(|report| 
            if looks_like_decreasing_report(report)
                {report.iter().map(|&x| x.neg()).collect()} 
            else 
                {report.clone()})
        .filter(|report| 
            is_safe(report, 0,false)
        )
        .count() as i32
}
pub fn solution(input: String) -> String {
    let reports: Vec<Vec<i32>> = input
                                    .lines()
                                    .map(|line| {
                                        line.to_string()
                                            .split_whitespace()
                                            .map(|x| x.parse::<i32>().unwrap())
                                            .collect()
                                    }).collect();

    format!("{}",number_of_safe_reports(reports))
}
