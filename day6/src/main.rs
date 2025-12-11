use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut counter_part1: u64 = 0;
    let instructions = fs::read_to_string("input.txt")?;
    let operands_1: Vec<u64> = instructions.lines().nth(0).unwrap()
        .split_ascii_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect();
    let operands_2: Vec<u64> = instructions.lines().nth(1).unwrap()
        .split_ascii_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect();
    let operands_3: Vec<u64> = instructions.lines().nth(2).unwrap()
        .split_ascii_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect();
    let operands_4: Vec<u64> = instructions.lines().nth(3).unwrap()
        .split_ascii_whitespace()
        .map(|value| value.parse::<u64>().unwrap())
        .collect();
    let operator: Vec<&str> = instructions.lines().nth(4).unwrap()
        .split_ascii_whitespace()
        .collect();
    
    for i in 0..operator.len() {
        if operator[i] == "*" {
            counter_part1 += operands_1[i] * operands_2[i] * operands_3[i] * operands_4[i]
        } else {
            counter_part1 += operands_1[i] + operands_2[i] + operands_3[i] + operands_4[i]
        }
    }
    println!("Solution part 1: {counter_part1}");
    Ok(())
}