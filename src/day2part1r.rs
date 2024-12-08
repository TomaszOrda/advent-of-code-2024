use std::ops::Neg;
fn looks_like_decreasing_report(report: &Vec<i32>) -> bool{
    report[0]>report[1]
}

fn is_safe(report: &Vec<i32>, id: usize) -> bool{
    if report.len()<id+2{
        return true
    }
    let difference = report[id+1] - report[id];
    if difference >= 1 && difference <= 3 {
        is_safe(report, id+1)
    }else{
        false
    }
}

fn number_of_safe_reports(reports:Vec<Vec<i32>>) ->i32{
    reports.iter()
        .map(|report| 
            if looks_like_decreasing_report(report)
                {report.iter().map(|&x| x.neg()).collect()} 
            else 
                {report.clone()})
        .filter(|report| 
            is_safe(report, 0)
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

    format!("{}", number_of_safe_reports(reports))
}
