use std::ops::Neg;


fn looks_like_decreasing_report(report: &[i32]) -> bool{
    report[0]>report[1]
}

fn number_of_safe_reports(reports:Vec<Vec<i32>>) ->i32{
    reports.iter()
        .map(|report| 
            if looks_like_decreasing_report(report)
                {report.iter().map(|&x| x.neg()).collect()} 
            else 
                {report.clone()})
        .filter(|report| 
            report.iter().fold(
                report[0]-1,
                |acc, x| 
                    if x - acc >= 1 && x - acc <= 3 
                        { *x }
                    else
                    { report[0] -8}
            ) > report[0]
        )
        .count() as i32
}
pub fn solution(input:String) -> String{
    
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
