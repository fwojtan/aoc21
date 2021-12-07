use std::io::{self, prelude::*, BufReader};
use std::fs::File;
use std::iter;


pub fn main() {
    part_one();
    part_two();
}


fn binary_string_to_int(string:String) -> i32 {
    let mut result:i32 = 0;
    for (i, character) in string.chars().rev().enumerate() {
        let base = i32::from(2).pow(i.try_into().unwrap());
        let multiplier:i32 =  character.to_string().parse().unwrap();
        result += base * multiplier;
    }

    result
}

fn unpack_input() -> io::Result<Vec<String>> {
    println!("Unpacking!");
    let file = File::open("data/input3.txt")?;
    let reader = BufReader::new(file);

    let mut inputs = Vec::new();

    for line in reader.lines(){
        //println!("{}", line?);
        inputs.push(line?)
    }

    Ok(inputs)
}

fn part_one() {
    let input = unpack_input().unwrap();
    let mut ones:Vec<i32> = iter::repeat(i32::from(0)).take(12).collect();
    let mut zeros:Vec<i32> = iter::repeat(i32::from(0)).take(12).collect();

    for bits in input {
        for (i, bit) in bits.chars().enumerate() {
            if bit == '1' {
                ones[i] += 1;
            } else if bit == '0' {
                zeros[i] += 1;
            }
        }
    }
    let mut gamma_string = "".to_string();
    let mut epsilon_string = "".to_string();
    for (num_ones, num_zeros) in ones.iter().zip(zeros.iter()){
        if num_ones > num_zeros {
            gamma_string += "1";
            epsilon_string += "0";
        } else if num_ones < num_zeros {
            gamma_string += "0";
            epsilon_string += "1";
        }
    }

    println!("gamma: {}, epsilon: {}", gamma_string, epsilon_string);

    //let gamma:i32 = gamma_string.parse().unwrap();
    //let espilon:i32 = epsilon_string.parse().unwrap();
    println!("{}", binary_string_to_int(gamma_string)*binary_string_to_int(epsilon_string));


}

fn part_two() {
    let input = unpack_input().unwrap();
    let mut oxygen = input.clone();
    let mut carbon = input.clone();
    
    for i in 0..input[0].len() { // need a condition to stop each when only one left
        
        if oxygen.len() > 1 {
            if get_most(oxygen.clone(), i) == '0' {
                oxygen = filter_list(oxygen, '0', usize::from(i));
            } else {
                oxygen = filter_list(oxygen, '1', usize::from(i));
            }
        }

        if carbon.len() > 1 {
            if get_most(carbon.clone(), i) == '0' {
                carbon = filter_list(carbon, '1', usize::from(i));
            } else {
                carbon = filter_list(carbon, '0', usize::from(i));
            }
        }

        if oxygen.len() < 5 {
            println!("Oxygen list:");
            for item in &oxygen {
                println!("{}", item);
            }
        } else {
            println!("Oxygen list:");
            for i in 0..4 {
                let item = oxygen.get(i);
                println!("{}", item.unwrap());
            }
        }
        if carbon.len() < 5 {
            println!("Carbon list:");
            for item in &carbon {
                println!("{}", item);
            }
        } else {
            println!("Carbon list:");
            for i in 0..4 {
                let item = carbon.get(i);
                println!("{}", item.unwrap());
            }
        }

        println!("Oxygen size: {}, Carbon size: {}", oxygen.len(), carbon.len());

    }

    let oxy_val = binary_string_to_int(oxygen.get(0).unwrap().to_string());
    let car_val = binary_string_to_int(carbon.get(0).unwrap().to_string());

    println!("Oxygen: {}", oxy_val);
    println!("Carbon: {}", car_val);

    println!("Answer: {}", oxy_val * car_val);



    // for bits in input {
    //     for (i, bit) in bits.chars().enumerate() {
    //         if bit == '1' {
    //             ones[i] += 1;
    //         } else if bit == '0' {
    //             zeros[i] += 1;
    //         }
    //     }
    // }
    // let mut gamma_string = "".to_string();
    // let mut epsilon_string = "".to_string();
    // for (num_ones, num_zeros) in ones.iter().zip(zeros.iter()){
    //     if num_ones > num_zeros {
    //         gamma_string += "1";
    //         epsilon_string += "0";
    //     } else if num_ones < num_zeros {
    //         gamma_string += "0";
    //         epsilon_string += "1";
    //     }
    // }

    // println!("gamma: {}, epsilon: {}", gamma_string, epsilon_string);

    // //let gamma:i32 = gamma_string.parse().unwrap();
    // //let espilon:i32 = epsilon_string.parse().unwrap();
    // println!("{}", binary_string_to_int(gamma_string)*binary_string_to_int(epsilon_string));

 
}

fn filter_list(data:Vec<String>, value:char, position:usize) -> Vec<String> {
    let mut new_vec:Vec<String> = Vec::new();
    println!("Filtering list of length {}", data.len());
    for string in data {
        if string.chars().nth(position).unwrap() == value {
            new_vec.push(string)
        }
    }
    println!("New filtered list of length {}", new_vec.len());

    new_vec
}

fn get_most(data:Vec<String>, index:usize) -> char {
    let mut ones:i32 = 0;
    let mut zeros:i32 = 0;
    for bits in &data {
        if bits.chars().nth(index).unwrap() == '1' {
            ones += 1;
        } else if bits.chars().nth(index).unwrap() == '0' {
            zeros += 1;
        }
    }
    println!("Calculated occurences - Ones: {}, Zeros: {}", ones, zeros);
    if ones > zeros {
        '1'
    } else if zeros > ones {
        '0'
    } else {
        'e'
    }
}
