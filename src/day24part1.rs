use std::collections::HashMap;
#[derive(Debug)]
enum Wire{
    Xor(String, String),
    Or(String, String),
    And(String, String),
    Val(bool)
}
fn eval(wire_name: &String, circuit: &HashMap<String, Wire>) -> bool{
    let wire = circuit.get(wire_name).unwrap();
    match wire{
        Wire::And(w1, w2) => eval(w1, circuit) && eval(w2, circuit),
        Wire::Or(w1, w2) => eval(w1, circuit) || eval(w2, circuit),
        Wire::Xor(w1, w2) => eval(w1, circuit) ^ eval(w2, circuit),
        Wire::Val(value) => *value
    }
}

pub fn solution(input: String) -> String { 
    let wires = input
        .lines()
        .take_while(|line| line!=&"")
        .map(
            |line|
            line.split(": ").collect::<Vec<&str>>())
        .map(|line| (line[0].to_string(), Wire::Val(line[1]=="1")));
    let operations = input
        .lines()
        .skip_while(|line| !line.contains("->"))
        .map(
            |line|
            line.split(" ").collect::<Vec<&str>>())
        .map(
            |line| {
                let arg1 = line[0].to_string();
                let arg2 = line[2].to_string();
                let res = line[4].to_string();
                let op = line[1];
                (res, match op{
                    "AND" => Wire::And(arg1, arg2),
                    "OR"  => Wire::Or (arg1, arg2),
                    "XOR" => Wire::Xor(arg1, arg2),
                    _ => panic!("Unknown operation {}", op)
                })
            });
    
    //We could remember each calculation — reduce gates to values. However it is unnecessary in part 1, and unhelpful in part 2 (unless we were to bruteforce it maybe)
    let circuit = wires.chain(operations).collect::<HashMap<String, Wire>>();

    let mut output = circuit.iter().filter(|wire| wire.0.starts_with("z")).map(|wire| wire.0).collect::<Vec<&String>>();
    output.sort();

    format!("{:?}",output.iter().map(|wire| eval(wire, &circuit)).rev().fold(0, |acc, b| acc*2 + b as u64 ))
} 

#[test]
fn basic_test() {
    let input = "x00: 1
                        x01: 1
                        x02: 1
                        y00: 0
                        y01: 1
                        y02: 0
                        
                        x00 AND y00 -> z00
                        x01 XOR y01 -> z01
                        x02 OR y02 -> z02".lines().map(|line| line.trim().to_string()).collect::<Vec<String>>().join("\r\n");
    assert_eq!(solution(input).parse::<u64>().unwrap(), 4)
}
