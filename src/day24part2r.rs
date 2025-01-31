// There is an assumption that the adder is design as expected, using xors, ands and one or for carry (that doesnt hold for the test case). Look at the diagram below.
/*
x_i ──────┬────►┌───┐                                 
          │     │XOR│────┬──────►┌───┐                
y_i ───┬───────►└───┘    │       │XOR│─────────► z_i  
       │  │              │  ┌───►└───┘                
       │  │              │  │                         
       │  │              │  │                         
       │  │              │  │                         
       │  └────►┌───┐    └──────►┌───┐                
       │        │AND│───┐   │    │AND│─┐              
       └───────►└───┘   │   ├───►└───┘ └─►┌──┐        
                        │   │             │OR│─► c_i+1
                        └────────────────►└──┘        
                            │                         
c_i ────────────────────────┘                         
*/
// In case of the solution not working, one is supposed to develop the repair_swaps function further. For me it was enough to cover only two cases.
// 
// Originally I have solved this problem by hand (while also writing a lot of code to be able to test swaps and pinpoint problematic wires).

use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Wire{
    Xor(String, String),
    Or(String, String),
    And(String, String),
    Val(bool)
}
struct Circuit{
    connections: HashMap<String, Wire>
}
impl Circuit{
    fn eval(&self, wire_name: &String) -> bool{
        let wire = self.connections.get(wire_name).unwrap();
        match wire{
            Wire::And(w1, w2) => self.eval(w1) && self.eval(w2),
            Wire::Or (w1, w2) => self.eval(w1) || self.eval(w2),
            Wire::Xor(w1, w2) => self.eval(w1) ^  self.eval(w2),
            Wire::Val(value) => *value
        }
    }
    
    fn _swap_wires(&mut self, wire1: &String, wire2: &String){
        let a = self.connections.get(wire1).unwrap().clone();
        let b = self.connections.get(wire2).unwrap().clone();
        *self.connections.get_mut(wire1).unwrap() = b;
        *self.connections.get_mut(wire2).unwrap() = a;
    }

    fn load(&mut self, var: char, val:u64){
        let val:Vec<bool> = (0..64).map(|shift| ((val>>shift) %2) == 1 ).collect();
        self.connections.iter_mut().filter(|x| x.0.chars().next().unwrap()==var).for_each(|(key, value)| *value=Wire::Val(val[key[1..=2].parse::<usize>().unwrap()]) );
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
        self.load('x', 0);
        self.load('y', val);
        let y_out = &self.output_u64();
        if  y_out == &val{
            true
        }else if y_out != &val{
            false
        }else{
            panic!("Case not covered")
        }
    }
    fn _test_bits(&mut self){
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
    fn children_xor(&self, wire1: &String,wire2: &String) -> String{
        self.connections
            .iter()
            .find(
                |(_name, wire)| 
                {
                    match wire{
                        Wire::Xor(a,b) => a == wire1 && b == wire2 || b == wire1 && a == wire2, 
                        _ => false
                    }
                })
            .unwrap().0.to_string()
    }
    fn children_and(&self, parent1: &String,parent2: &String) -> String{
        self.connections
            .iter()
            .find(
                |(_name, wire)| 
                {
                    match wire{
                        Wire::And(a,b) => a == parent1 && b == parent2 || b == parent1 && a == parent2,
                        _ => false
                    }
                })
            .unwrap().0.to_string()
    }
    fn children_wires(&self, parent:&String)-> Vec<String>{
        self.connections
            .iter()
            .filter(
                |(_name, wire)| 
                {
                    match wire{
                        Wire::And(a,b) => a == parent || b == parent,
                        Wire::Or (a,b) => a == parent || b == parent,
                        Wire::Xor(a,b) => a == parent || b == parent, 
                        _ => false
                    }
                })
            .map(|(name, _)| name.to_string())
            .collect::<Vec<String>>()
    }
    fn _parent_wires(&self, input:&String)-> Vec<String>{
        match self.connections.get(input).unwrap(){
            Wire::And(a,b) => vec![a.to_string(),b.to_string()],
            Wire::Or (a,b) => vec![a.to_string(),b.to_string()],
            Wire::Xor(a,b) => vec![a.to_string(),b.to_string()], 
            _ => vec![]
        }
    }
    fn swap_to_repair(&self, bit:u32) -> (String, String) {
        let x = format!("x{bit}");
        let y = format!("y{bit}");
        let x_xor_y = self.children_xor(&x,&y);
        let x_and_y = self.children_and(&x,&y);
        let mut x_xor_y_xor_c_candidates = self.children_wires(&x_xor_y)
            .into_iter()
            .filter(|x| matches!(self.connections.get(x).unwrap(), Wire::Xor(_, _)));
        let z = format!("z{bit}");

        // Output wire swapped
        if let Some(x_xor_y_xor_c) = x_xor_y_xor_c_candidates.next(){
            if  x_xor_y_xor_c != format!("z{bit}"){
                return (x_xor_y_xor_c, z)
            }
        }

        // Base case
        (x_xor_y, x_and_y)
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
    
    let mut circuit = wires.chain(operations).collect::<Circuit>();

    let _initial_x = circuit.get_variable_u64('x');
    let _initial_y = circuit.get_variable_u64('y');

    let wrong_bits = (0..45).filter(|&bit| !circuit.test_bit(bit)).collect::<Vec<u32>>();
    let mut swaps = wrong_bits
        .iter()
        .map(|&bit| circuit.swap_to_repair(bit))
        .flat_map(|(a,b)| vec![a, b])
        .collect::<Vec<String>>();
    swaps.sort();
    swaps.join(",")
} 