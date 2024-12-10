@echo off

set DayNumber=%1

type NUL > Day_%DayNumber%.txt
type NUL > Day_%DayNumber%_test.txt

echo. >> src\day%DayNumber%part1.rs
echo pub fn solution(input: String) -^> String { >> src\day%DayNumber%part1.rs
echo. >> src\day%DayNumber%part1.rs
echo     format!("{:?}",input) >> src\day%DayNumber%part1.rs
echo } >> src\day%DayNumber%part1.rs

copy src\day%DayNumber%part1.rs src\day%DayNumber%part2.rs
