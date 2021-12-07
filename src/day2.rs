use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub fn main() -> io::Result<()> {
    //part_one()
    part_two()
}

struct Instruction {
    direction: i32, // 1 or -1
    magnitude: i32,
    axis: String, // "depth" or "dist"
}

impl Instruction {
    fn new_from_string(input:String) -> Instruction {
        let split_input: Vec<&str> = input.split(" ").collect();
        let (direction, axis) = match split_input[0] {
            "up" => (-1, "depth".to_string()),
            "down" => (1, "depth".to_string()),
            "forward" => (1, "dist".to_string()),
            _ => (0, "error".to_string())
        };
        Instruction{direction:direction, magnitude:split_input[1].parse::<i32>().unwrap(), axis:axis}
    }

    fn new_position(&self, mut position:HashMap<String, i32>) -> HashMap<String, i32> {
        *position.get_mut(&self.axis).unwrap() += self.direction*self.magnitude;
        position
    }

    fn new_position_part2(&self, mut position:HashMap<String, i32>) -> HashMap<String, i32> {
        if self.axis == "depth".to_string() {
            *position.get_mut(&"aim".to_string()).unwrap() += self.direction*self.magnitude;
        } else if self.axis == "dist".to_string() {
            *position.get_mut(&"dist".to_string()).unwrap() += self.direction*self.magnitude;
            *position.get_mut(&"depth".to_string()).unwrap() += position[&"aim".to_string()]*self.direction*self.magnitude;
        }



        position
    }
}

fn parse_instructions() -> io::Result<Vec<Instruction>> {
    println!("Starting!");
    let file = File::open("data/input2.txt")?;
    let reader = BufReader::new(file);

    let mut instructions = Vec::new();

    for line in reader.lines(){
        //println!("{}", line?);
        instructions.push(Instruction::new_from_string(line?))
    }

    Ok(instructions)
}

fn part_one() -> io::Result<()> {
    
    let instructions = parse_instructions().unwrap();

    let mut position = HashMap::new();
    position.insert("dist".to_string(), 0);
    position.insert("depth".to_string(), 0);
    for instruction in instructions{
        println!("{}, {}, {}", instruction.direction, instruction.magnitude, instruction.axis);
        position = instruction.new_position(position);
        println!("Pos updated to dist: {}, depth: {}", position[&"dist".to_string()], position[&"depth".to_string()])
    }

    println!("Result: {}", position.get(&"depth".to_string()).unwrap() * position.get(&"dist".to_string()).unwrap());

    Ok(())
}

fn part_two() -> io::Result<()> {
    let instructions = parse_instructions().unwrap();

    let mut position = HashMap::new();
    position.insert("dist".to_string(), 0);
    position.insert("depth".to_string(), 0);
    position.insert("aim".to_string(), 0);
    for instruction in instructions{
        println!("{}, {}, {}", instruction.direction, instruction.magnitude, instruction.axis);
        position = instruction.new_position_part2(position);
        println!("Pos updated to dist: {}, depth: {}", position[&"dist".to_string()], position[&"depth".to_string()])
    }

    println!("Result: {}", position.get(&"depth".to_string()).unwrap() * position.get(&"dist".to_string()).unwrap());


    Ok(())
}
