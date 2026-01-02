use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut counter_part1: u64 = 0;
    let file = fs::read_to_string("input.txt")?;
    let mut instructions: Vec<&str>= file.lines().collect();
    let operators: Vec<&str> = instructions.pop().unwrap()
        .split_ascii_whitespace()
        .collect();
    let operands: Vec<Vec<u64>> = instructions.iter().map(
        |&line| 
        line.split_ascii_whitespace().map(
            |value| value.parse::<u64>().unwrap()
        ).collect()
    ).collect();

    let mut operands_t: Vec<Vec<u64>> = Vec::new();
    for x in 0..operands[0].len() {
        let mut column: Vec<u64> = Vec::new();
        for y in 0..operands.len() {
            column.push(operands[y][x]);
        }
        operands_t.push(column);
    }
    
    for i in 0..operators.len() {
        if operators[i] == "*" {
            counter_part1 += operands_t[i].iter().copied().reduce(|acc, e| acc * e).unwrap()
        } else {
            counter_part1 += operands_t[i].iter().copied().reduce(|acc, e| acc + e).unwrap()
        }
    }
    println!("Solution part 1: {counter_part1}");
    Ok(())
}