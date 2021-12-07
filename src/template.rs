use std::io::{self, prelude::*, BufReader};
use std::fs::File;

fn main() {
    let input = unpack_input();
    
}


fn unpack_input() -> io::Result<Vec<String>> {
    println!("Starting!");
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut inputs = Vec::new();

    for line in reader.lines(){
        //println!("{}", line?);
        inputs.push(line?)
    }

    Ok(inputs)
}