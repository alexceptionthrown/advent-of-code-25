use std::fs;
use std::error::Error;
use std::cmp::max;

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = fs::read_to_string("input.txt")?;
    let lines = instructions.lines();
    let mut counter_part1: u64 = 0;
    let mut counter_part2: u64 = 0;
    for line in lines{
        counter_part1 += compute_max_joltage(line, 2);
        counter_part2 += compute_max_joltage(line, 12);
    }
    println!("Solution part 1: {counter_part1}\nSolution part 2: {counter_part2}");
    Ok(())
}

fn compute_max_joltage(available_batteries: &str, max_usable_batteries: usize) -> u64 {
    let mut current_best_batteries: Vec<Option<u32>> = vec![None; max_usable_batteries];
    let mut current_len = 0;
    for char in available_batteries.chars() {
        let val: u32 = char.to_digit(10).unwrap();
        let mut assigned = false;
        let mut i = max(0, -(available_batteries.len() as i32) + current_len + current_best_batteries.len() as i32);
        while !assigned && i < current_best_batteries.len() as i32 {
            assigned = assigned_if_better(&mut current_best_batteries[i as usize], val);
            i += 1;
        }
        if assigned {
            for j in i..current_best_batteries.len() as i32 {
                current_best_batteries[j as usize] = None
            }
        }
        // reset for next loop
        current_len += 1;
    }
    let chosen_combo: String = current_best_batteries.iter().fold("".to_string(), |acc, e| acc + &e.unwrap().to_string());
    chosen_combo.parse::<u64>().unwrap()
}

fn assigned_if_better(stored_val: &mut Option<u32>, val: u32) -> bool {
    match *stored_val {
        None => {
            *stored_val = Some(val);
            true
        },
        Some(x) => if val > x {
            *stored_val = Some(val);
            true
        } else {
            false
        }
    }
}
