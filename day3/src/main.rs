use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = fs::read_to_string("input.txt")?;
    let lines = instructions.lines();
    let mut counter_part1 = 0;
    for line in lines{
        let mut decimal: Option<u32> = None;
        let mut unit: Option<u32> = None;
        let mut current_len = 0;
        let mut assigned = false;
        for char in line.chars() {
            let val: u32 = char.to_digit(10).unwrap();
            if current_len < line.len() - 1 {
                assigned = assigned_if_better(&mut decimal, val);
            }
            if assigned {
                unit = None;
            } else {
                assigned_if_better(&mut unit, val);
            }

            // reset for next loop
            current_len += 1;
            assigned = false;
        }
        let chosen_combo = decimal.unwrap().to_string() + &unit.unwrap().to_string();
        counter_part1 += chosen_combo.parse::<u32>().unwrap();
    }
    println!("{counter_part1}");
    Ok(())
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
