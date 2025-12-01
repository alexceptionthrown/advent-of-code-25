use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let instructions = fs::read_to_string("input1.txt")?;
    let lines = instructions.lines();
    let mut pos: i32 = 50;
    let mut counter_part1: i32 = 0;
    let mut counter_part2: i32 = 0;
    for line in lines{
        let (dir, amount)= line.split_at(1);
        let amount: i32 = amount.parse().unwrap();
        if dir == "R" {
            counter_part2 += (pos + amount) / 100;
            pos = (pos + amount) % 100;
        } else {
            counter_part2 += (((100 - pos) % 100) + amount) / 100;
            pos = (100 - (((100 - pos) + amount) % 100)) % 100;
        }
        if pos == 0 {
            counter_part1+=1;
        }
        }
    println!("Solution part 1: {counter_part1}\nSolution part 2: {counter_part2}");
    Ok(())
}
