use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
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

    let instructions_chars: Vec<Vec<char>> = instructions.iter().map(|&line| line.chars().collect()).collect();
    let mut column_operands: Vec<Vec<u64>> = Vec::new();
    let mut buffer: Vec<u64> = Vec::new();
    for x in 0..instructions_chars[0].len() {
        let mut char_vec: Vec<char> = Vec::new();
        for y in 0..instructions_chars.len() {
            char_vec.push(instructions_chars[y][x]);
        }
        let operand_string: String = char_vec.into_iter().collect();
        if operand_string.trim().is_empty() {
            column_operands.push(buffer);
            buffer = Vec::new();
        } else {
            buffer.push(operand_string.trim().parse::<u64>().unwrap());
        }
    }
    column_operands.push(buffer);

    
    let counter_part1 = compute_results(&operators, operands_t);
    let counter_part2 = compute_results(&operators, column_operands);
    println!("Solution part 1: {counter_part1}\nSolution part 2: {counter_part2}");
    Ok(())
}

fn compute_results(operators: &Vec<&str>, operands: Vec<Vec<u64>>) -> u64 {
    let mut counter: u64 = 0;
    for i in 0..operators.len() {
        if operators[i] == "*" {
            counter += operands[i].iter().copied().reduce(|acc, e| acc * e).unwrap()
        } else {
            counter += operands[i].iter().copied().reduce(|acc, e| acc + e).unwrap()
        }
    }
    counter
}