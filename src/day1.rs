use std::fs::File;
use std::io::{self, prelude::*, BufReader};

pub fn main() -> io::Result<()> {
    let file = File::open("data/input1.txt")?;
    let reader = BufReader::new(file);
    let mut last_line = 0;
    let mut line_before_last = 0;
    let mut last_sum = 0;
    let mut counter = -1; // cheating to exclude the first input
    let mut counter2 = -3;

    for line in reader.lines(){
        let line_val = line?.parse::<i32>().unwrap();
        let outcome1 = if line_val > last_line {counter += 1; "increased"} else {"decreased"};
        
        let sum = line_val + last_line + line_before_last;

        let outcome2 = if sum > last_sum {
            counter2 += 1;
            "rolling increased"
        } else {
            "rolling decreased"
        };
        
        println!("{}, {}, {}, \t - {}", line_val, last_line, outcome1, counter);
        println!("\x1b[32m {}, {}, {}, \t - {}\x1b[0m", sum, last_sum, outcome2, counter2);

        line_before_last = last_line;
        last_line = line_val;
        last_sum = sum;
    }

    

    Ok(())
}
