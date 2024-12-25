use std::{collections::HashMap, io};
#[derive(Debug, Clone)]
enum Wire{
    XOR(String, String),
    OR(String, String),
    AND(String, String),
    VAL(bool)
}
struct Circuit{
    connections: HashMap<String, Wire>
}
impl Circuit{
    fn eval(&self, wire_name: &String) -> bool{
        let wire = self.connections.get(wire_name).unwrap();
        match wire{
            Wire::AND(w1, w2) => self.eval(&w1) && self.eval(&w2),
            Wire::OR (w1, w2) => self.eval(&w1) || self.eval(&w2),
            Wire::XOR(w1, w2) => self.eval(&w1) ^  self.eval(&w2),
            Wire::VAL(value) => *value
        }
    }
    
    fn swap_wires(&mut self, wire1: &String, wire2: &String){
        let a = self.connections.get(wire1).unwrap().clone();
        let b = self.connections.get(wire2).unwrap().clone();
        *self.connections.get_mut(wire1).unwrap() = b;
        *self.connections.get_mut(wire2).unwrap() = a;
    }

    fn load(&mut self, var: char, val:u64){
        let val:Vec<bool> = (0..64).map(|shift| ((val>>shift) %2) == 1 ).collect();
        self.connections.iter_mut().filter(|x| x.0.chars().next().unwrap()==var).for_each(|(key, value)| *value=Wire::VAL(val[key[1..=2].parse::<usize>().unwrap()]) );
    }
    fn get_variable(&self, v:char) -> Vec<bool>{
        let mut output_wires = self.connections.iter().filter(|wire| wire.0.starts_with(v)).map(|wire| wire.0).collect::<Vec<&String>>();
        output_wires.sort();

        output_wires.iter().map(|wire| self.eval(wire)).rev().collect::<Vec<bool>>()
    }
    fn _get_variable_bits(&self, v:char) -> String{
        self.get_variable(v).iter().map(|&b| format!("{}", b as u32)  ).collect::<Vec<String>>().join("")
    }
    fn get_variable_u64(&self, v:char)-> u64{
        self.get_variable(v).iter().fold(0, |acc, &b| acc*2 + b as u64 )
    }
    fn _output(&self)-> Vec<bool>{
        self.get_variable('z')
    }
    fn _output_bits(&self)-> String{
        self._get_variable_bits('z')
    }
    fn output_u64(&self)-> u64{
        self.get_variable_u64('z')
    }
    fn test_bit(&mut self, bit:u32) -> bool{
        let val = 1<<bit;
        // self.load('x', val);
        // self.load('y', 0);
        // let x_out = &self.output_u64();
        self.load('x', 0);
        self.load('y', val);
        let y_out = &self.output_u64();
        if  y_out == &val{ //x_out == &val &&
            true
        }else if y_out != &val{ //&& x_out != &val
            println!("For bit number: {bit}");
            println!("Expected: {:045b}",val);
            println!("Result:   {:045b}",y_out);
            false
        }else{
            panic!("Case not covered")
        }
    }
    fn test_bits(&mut self){
        (0..45)
            .for_each(
                |bit| 
                {self.test_bit(bit);});
    }
    fn _test_numbers(&mut self){
        let numbers = [123456789, 987654321, 4294967295, 1125899906842623, 281474976710655, 1099511627775, 8388607, 1023, 65535,4398046511103];
        for input_x in numbers{
            for input_y in numbers{
                self.load('x', input_x);
                self.load('y', input_y);
                if self.output_u64()!= input_x+input_y{
                    println!("x:        {:045b}",input_x);
                    println!("y:        {:045b}",input_y);
                    println!("Expected: {:045b}",input_x+input_y);
                    println!("Result:   {:045b}",self.output_u64());
                }
            }
        }
    }
    fn children_wires(&self, input:&String)-> Vec<String>{
        self.connections
            .iter()
            .filter(
                |(_name, wire)| 
                {
                    match wire{
                        Wire::AND(a,b) => a == input || b == input,
                        Wire::OR (a,b) => a == input || b == input,
                        Wire::XOR(a,b) => a == input || b == input, 
                        _ => false
                    }
                })
            .map(|(name, _)| name.to_string())
            .collect::<Vec<String>>()
    }
    fn parent_wires(&self, input:&String)-> Vec<String>{
        match self.connections.get(input).unwrap(){
            Wire::AND(a,b) => vec![a.to_string(),b.to_string()],
            Wire::OR (a,b) => vec![a.to_string(),b.to_string()],
            Wire::XOR(a,b) => vec![a.to_string(),b.to_string()], 
            _ => vec![]
        }
    }
}
impl FromIterator<(String, Wire)> for Circuit {
    fn from_iter<I: IntoIterator<Item = (String, Wire)>>(iter: I) -> Self {
        Self {
            connections: iter.into_iter().collect(),
        }
    }
}
pub fn solution(input: String) -> String { 
    let wires = input
        .lines()
        .take_while(|line| line!=&"")
        .map(
            |line|
            line.split(": ").collect::<Vec<&str>>())
        .map(|line| (line[0].to_string(), Wire::VAL(line[1]=="1")));
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
                    "AND" => Wire::AND(arg1, arg2),
                    "OR"  => Wire::OR (arg1, arg2),
                    "XOR" => Wire::XOR(arg1, arg2),
                    _ => panic!("Unknown operation {}", op)
                })
            });
    
    let mut circuit = wires.chain(operations).collect::<Circuit>();

    let _initial_x = circuit.get_variable_u64('x');
    let _initial_y = circuit.get_variable_u64('y');


    let mut swaps: Vec<(String, String)> = vec![];
    loop{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        match command[0].as_str(){
            "quit" => break,
            "bitTest" =>circuit.test_bits(),
            // "numberTest" => circuit.test_numbers(),
            "show" =>{
                match command[1].as_str() {
                    "swaps"     => println!("{}", swaps.iter().flat_map(|(a,b)| vec![a.clone(), b.clone()]).collect::<Vec<String>>().join(",")),
                    "children"  => println!("{:?}", circuit.children_wires(&command[2])),
                    "parents"   => println!("{:?}", circuit.parent_wires(  &command[2])),
                    _ => println!("Unknown show parameter")
                }
            }
            "swap" => {
                let swap = (command[1].clone(), command[2].clone());
                let swap_reversed = (swap.0.clone(), swap.1.clone());
                circuit.swap_wires(&swap.0, &swap.1);
                if swaps.contains(&swap) || swaps.contains(&swap_reversed){
                    let pos = swaps.iter().position(|x| x == &swap || x == &swap_reversed).unwrap();
                    swaps.remove(pos);
                }else{
                    swaps.push((command[1].clone(), command[2].clone()));
                }
            },
            _ => println!("Unknown command")
        }
    }
    let mut swaps_flat =  swaps.iter().flat_map(|(a,b)| vec![a.to_string(), b.to_string()]).collect::<Vec<String>>();
    swaps_flat.sort();
    swaps_flat.join(",")
} 